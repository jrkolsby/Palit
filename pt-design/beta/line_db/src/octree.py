NODE_CAPACITY = 1

class Window:
    def __init__(self, x, y, z, half_dim):
        self.x = x
        self.y = y
        self.z = z

        self.half_dim = half_dim

        self.min_x = x - half_dim
        self.max_x = x + half_dim
        self.min_y = y - half_dim
        self.max_y = y + half_dim
        self.min_z = z - half_dim
        self.max_z = z + half_dim

    def contains(self, p):
        return p.x >= self.min_x and \
            p.x <= self.max_x and \
            p.y >= self.min_y and \
            p.y <= self.max_y and \
            p.z >= self.min_z and \
            p.z <= self.max_z

    def intersects(self, win):
        return self.max_x > win.min_x and \
            self.min_x < win.max_x and \
            self.max_y > win.min_y and \
            self.min_y < win.max_y and \
            self.max_z > win.min_z and \
            self.min_z < win.max_z

    def __str__(self):
        return '{0},{1},{2} ({3})'.format(self.x, self.y, self.z, self.half_dim)

class Line:
    def __init__(self, x, y, z, length, char):
        self.x = x
        self.y = y
        self.z = z
        self.length = length
        self.char = char

    def __str__(self):
        return '{0},{1},{2}:{3} ({4})'.format(self.x, self.y, self.z, self.length, self.char)

class OctNode:

    # 0 < x < SIZE
    # 0 < y < SIZE
    # -90 < z < 90 (theta)

    def __init__(self, win):
        self.win = win

        self.data = []

        self.children = [
            None, # x' > x, y' > y, z' > z
            None, # x' < x, y' > y, z' > z
            None, # x' > x, y' < y, z' > z
            None, # x' < x, y' < y, z' > z
            None, # x' > x, y' > y, z' < z
            None, # x' < x, y' > y, z' < z
            None, # x' > x, y' < y, z' < z
            None, # x' < x, y' < y, z' < z
        ]

    # Return True if success
    def insert(self, point):
        if not self.win.contains(point):
            return False

        if len(self.data) < NODE_CAPACITY and self.children[0] is None:
            self.data.append(point)
            return True

        if self.children[0] is None:
            self.subdivide()

        for child in self.children:
            if child.insert(point):
                return True

        return False

    def subdivide(self):
        dim = self.win.half_dim // 2
        self.children = [
            OctNode(Window(self.win.x + dim, self.win.y + dim, self.win.z + dim, dim)),
            OctNode(Window(self.win.x - dim, self.win.y + dim, self.win.z + dim, dim)),
            OctNode(Window(self.win.x + dim, self.win.y - dim, self.win.z + dim, dim)),
            OctNode(Window(self.win.x - dim, self.win.y - dim, self.win.z + dim, dim)),
            OctNode(Window(self.win.x + dim, self.win.y + dim, self.win.z - dim, dim)),
            OctNode(Window(self.win.x - dim, self.win.y + dim, self.win.z - dim, dim)),
            OctNode(Window(self.win.x + dim, self.win.y - dim, self.win.z - dim, dim)),
            OctNode(Window(self.win.x - dim, self.win.y - dim, self.win.z - dim, dim))
        ]

        for point in self.data:
            self.insert(point)

        self.data = []

    def query(self, win):
        points = []

        if not win.intersects(self.win):
            return points

        for point in self.data:
            if win.contains(point):
                points.append(point)

        if self.children[0] is None:
            return points

        for child in self.children:
            points += child.query(win)

        return points

    def __str__(self):
        str_acc = '{0}:[\n'.format(self.win)
        for point in self.data:
            str_acc = '{0}{1}\n'.format(str_acc, str(point))
        str_acc = '{0}]\n'.format(str_acc)
        for child in self.children:
            if child is None:
                break
            str_acc = '{0}\n{1}'.format(str_acc, str(child))
        return str_acc

    # preorder traversal
    def traverse(self, func):
        func(self.data)
        for child in self.children:
            if child is None:
                break
            child.traverse(func)

    def write(self, file, depth):
        file.write('{0}<node x="{1}" y="{2}" z="{3}" dim="{4}">\n'
            .format('\t' * depth, self.win.x, self.win.y, self.win.z, self.win.half_dim))
        for child in self.children:
            # Leaf node, print line contents
            if child is None:
                for line in self.data:
                    escaped_char = {
                        '"': '&quot;',
                        "'": '&apos;',
                        '>': '&lt;',
                        '<': '&gt;',
                        '&': '&amp;',
                    }.get(line.char, line.char)
                    file.write('{0}<line x="{1}" y="{2}" z="{3}" len="{4}" char="{5}" />\n'
                        .format('\t' * (depth + 1), line.x, line.y, line.z, line.length, escaped_char))
                break
            child.write(file, depth+1)
        file.write('{0}</node>\n'.format('\t' * depth))

# angle values need to have the same range as coords

'''
root = OctNode(Window(128, 128, 128, 128))

root.insert(Line(0, 51, 1, 1))
root.insert(Line(2, 4, 15, 2))
root.insert(Line(0, 0, 15, 2))
root.insert(Line(120, 0, 150, 3))
root.insert(Line(140, 0, 150, 4))
root.insert(Line(180, 70, 150, 5))
root.insert(Line(60, 30, 100, 6))
root.insert(Line(70, 20, 90, 7))
root.insert(Line(80, 100, 30, 8))
root.insert(Line(80, 90, 10, 9))

result = root.query(Window(60, 20, 90, 20))

print(str(root))

print(list(map(lambda m: m.data, result)))
'''
