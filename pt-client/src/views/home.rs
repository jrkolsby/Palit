use cursive::wrap_impl;
use cursive::view::{View, ViewWrapper, Scrollable};
use cursive::views::{LinearLayout, DummyView};
use cursive::theme::{Color, BaseColor};

use crate::components::{Splash, SplashAsset, ColorButton};

use crate::utils::HomeState;

pub struct Home<T: View> {
    state: HomeState,
    layout: T
}

impl<'a> Home<LinearLayout> {
    pub fn new(default_state: HomeState) -> Self {
        let mut home = Home {
            state: default_state,
            layout: LinearLayout::vertical()
        };

        let mut projects_view = LinearLayout::vertical();
        
        let colors: Vec<Color> = vec![
            Color::Light(BaseColor::Yellow),
            Color::Light(BaseColor::Green),
            Color::Light(BaseColor::Blue),
            Color::Light(BaseColor::Magenta),
        ];

        for (i, project) in home.state.projects.iter().enumerate() {
            let color = colors[i % colors.len()];
            projects_view = projects_view
                .child(ColorButton::new(color, project.to_string(), false))
                .child(DummyView);
        };

        home.layout = home.layout
            .child(Splash::new(SplashAsset::Logo, &home.state.motd))
            .child(ColorButton::new(Color::Light(BaseColor::Red), "+ NEW".to_string(), true))
            .child(DummyView)
            .child(projects_view.scrollable());

        home
    }
}

impl<T: View> ViewWrapper for Home<T> {
    wrap_impl!(self.layout: T);
}