use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Color {
    No,
    Yes,
    Auto,
}

impl Color {
    /**
     * Evaluate if colors are supported
     */
    pub fn is_enabled(self) -> bool {
        match self {
            Color::No => false,
            Color::Yes => true,
            Color::Auto => supports_color::on(supports_color::Stream::Stdout).is_some(),
        }
    }
}

/**
 * Installs color_eyre, if Color is disabled we use an empty theme to disable error colors.
 */
pub fn install_color_eyre(color: Color) -> color_eyre::Result<(), color_eyre::Report> {
    if color.is_enabled() {
        color_eyre::install()
    } else {
        // Use an empty theme to disable error coloring
        color_eyre::config::HookBuilder::new()
            .theme(color_eyre::config::Theme::new())
            .install()
    }
}
