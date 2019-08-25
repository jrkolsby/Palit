use cursive::wrap_impl;
use cursive::view::{View, ViewWrapper, Scrollable};
use cursive::views::{LinearLayout, DummyView};
use cursive::theme::{Color, BaseColor};

use crate::components::{Splash, SplashAsset, ColorButton};

pub struct Home<T: View> {
    state: HomeState,
    layout: T
}

#[derive(Clone)]
pub struct HomeState {
    pub openProject: i32, 
    pub projects: Vec<String>,
    pub motd: String,
}

impl<'a> Home<LinearLayout> {
    pub fn new(default_state: HomeState) -> Self {
        let mut home = Home {
            state: default_state,
            layout: LinearLayout::vertical()
        };

        home.layout = home.layout
            .child(Splash::new(SplashAsset::Logo, &home.state.motd))
            .child(DummyView)
            .child(projects_view.scrollable());

        home
    }
}

impl<T: View> ViewWrapper for Home<T> {
    wrap_impl!(self.layout: T);
}