use crate::{executor, Application, Command, Element, Settings, Subscription};

/// A sandboxed [`Application`].
///
/// If you are a just getting started with the library, this trait offers a
/// simpler interface than [`Application`].
///
/// Unlike an [`Application`], a [`Sandbox`] cannot run any asynchronous
/// actions or be initialized with some external flags. However, both traits
/// are very similar and upgrading from a [`Sandbox`] is very straightforward.
///
/// Therefore, it is recommended to always start by implementing this trait and
/// upgrade only once necessary.
///
/// [`Application`]: trait.Application.html
/// [`Sandbox`]: trait.Sandbox.html
/// [`Command`]: struct.Command.html
/// [`Command::none`]: struct.Command.html#method.none
///
/// # Examples
/// [The repository has a bunch of examples] that use the [`Sandbox`] trait:
///
/// - [`bezier_tool`], a Paint-like tool for drawing Bézier curves using
/// [`lyon`].
/// - [`counter`], the classic counter example explained in [the overview].
/// - [`custom_widget`], a demonstration of how to build a custom widget that
/// draws a circle.
/// - [`geometry`], a custom widget showcasing how to draw geometry with the
/// `Mesh2D` primitive in [`iced_wgpu`].
/// - [`pane_grid`], a grid of panes that can be split, resized, and
/// reorganized.
/// - [`progress_bar`], a simple progress bar that can be filled by using a
/// slider.
/// - [`styling`], an example showcasing custom styling with a light and dark
/// theme.
/// - [`svg`], an application that renders the [Ghostscript Tiger] by leveraging
/// the [`Svg` widget].
/// - [`tour`], a simple UI tour that can run both on native platforms and the
/// web!
///
/// [The repository has a bunch of examples]: https://github.com/hecrj/iced/tree/0.1/examples
/// [`bezier_tool`]: https://github.com/hecrj/iced/tree/0.1/examples/bezier_tool
/// [`counter`]: https://github.com/hecrj/iced/tree/0.1/examples/counter
/// [`custom_widget`]: https://github.com/hecrj/iced/tree/0.1/examples/custom_widget
/// [`geometry`]: https://github.com/hecrj/iced/tree/0.1/examples/geometry
/// [`pane_grid`]: https://github.com/hecrj/iced/tree/0.1/examples/pane_grid
/// [`progress_bar`]: https://github.com/hecrj/iced/tree/0.1/examples/progress_bar
/// [`styling`]: https://github.com/hecrj/iced/tree/0.1/examples/styling
/// [`svg`]: https://github.com/hecrj/iced/tree/0.1/examples/svg
/// [`tour`]: https://github.com/hecrj/iced/tree/0.1/examples/tour
/// [`lyon`]: https://github.com/nical/lyon
/// [the overview]: index.html#overview
/// [`iced_wgpu`]: https://github.com/hecrj/iced/tree/0.1/wgpu
/// [`Svg` widget]: widget/svg/struct.Svg.html
/// [Ghostscript Tiger]: https://commons.wikimedia.org/wiki/File:Ghostscript_Tiger.svg
///
/// ## A simple "Hello, world!"
///
/// If you just want to get started, here is a simple [`Sandbox`] that
/// says "Hello, world!":
///
/// ```no_run
/// use iced::{Element, Sandbox, Settings, Text};
///
/// pub fn main() {
///     Hello::run(Settings::default())
/// }
///
/// struct Hello;
///
/// impl Sandbox for Hello {
///     type Message = ();
///
///     fn new() -> Hello {
///         Hello
///     }
///
///     fn title(&self) -> String {
///         String::from("A cool application")
///     }
///
///     fn update(&mut self, _message: Self::Message) {
///         // This application has no interactions
///     }
///
///     fn view(&mut self) -> Element<Self::Message> {
///         Text::new("Hello, world!").into()
///     }
/// }
/// ```
pub trait Sandbox {
    /// The type of __messages__ your [`Sandbox`] will produce.
    ///
    /// [`Sandbox`]: trait.Sandbox.html
    type Message: std::fmt::Debug + Send;

    /// Initializes the [`Sandbox`].
    ///
    /// Here is where you should return the initial state of your app.
    ///
    /// [`Sandbox`]: trait.Sandbox.html
    fn new() -> Self;

    /// Returns the current title of the [`Sandbox`].
    ///
    /// This title can be dynamic! The runtime will automatically update the
    /// title of your application when necessary.
    ///
    /// [`Sandbox`]: trait.Sandbox.html
    fn title(&self) -> String;

    /// Handles a __message__ and updates the state of the [`Sandbox`].
    ///
    /// This is where you define your __update logic__. All the __messages__,
    /// produced by user interactions, will be handled by this method.
    ///
    /// [`Sandbox`]: trait.Sandbox.html
    fn update(&mut self, message: Self::Message);

    /// Returns the widgets to display in the [`Sandbox`].
    ///
    /// These widgets can produce __messages__ based on user interaction.
    ///
    /// [`Sandbox`]: trait.Sandbox.html
    fn view(&mut self) -> Element<'_, Self::Message>;

    /// Runs the [`Sandbox`].
    ///
    /// On native platforms, this method will take control of the current thread
    /// and __will NOT return__.
    ///
    /// It should probably be that last thing you call in your `main` function.
    ///
    /// [`Sandbox`]: trait.Sandbox.html
    fn run(settings: Settings<()>)
    where
        Self: 'static + Sized,
    {
        <Self as Application>::run(settings)
    }
}

impl<T> Application for T
where
    T: Sandbox,
{
    type Executor = executor::Null;
    type Flags = ();
    type Message = T::Message;

    fn new(_flags: ()) -> (Self, Command<T::Message>) {
        (T::new(), Command::none())
    }

    fn title(&self) -> String {
        T::title(self)
    }

    fn update(&mut self, message: T::Message) -> Command<T::Message> {
        T::update(self, message);

        Command::none()
    }

    fn subscription(&self) -> Subscription<T::Message> {
        Subscription::none()
    }

    fn view(&mut self) -> Element<'_, T::Message> {
        T::view(self)
    }
}
