use iced::{advanced::{layout, mouse, renderer, widget::Tree, Layout, Widget}, Element, Length, Rectangle, Renderer, Size, Theme};

/// Works to convert `Option<T>` to `Element<'a, Message, Theme, Renderer>`
///
/// iced versions after 0.13.1 have a `From` impl so they don't need this.
///
/// Usage:
///
///     use iced_optional_element_shim::to_elem;
///
///     fn view(&self) -> Element<'a, Message> {
///         column!(
///            text("Example header"),
///
///            if true {
///                to_elem(Some(text("this element will display")))
///            } else {
///                to_elem::<Message, iced::widget::Text>(None)
///            },
///
///            if false {
///                to_elem(Some(text("this element will not display")))
///            } else {
///                to_elem::<Message, iced::widget::Text>(None)
///            },
///
///            text("footer"),
///        )
///     }
///
pub fn to_elem<'a, Message, T: Into<Element<'a, Message, Theme, Renderer>>>(element: Option<T>) -> Element<'a, Message, Theme, Renderer> {
    struct Void;

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Void
    where Renderer: iced::advanced::Renderer
    {
        fn size(&self) -> Size<Length> {
            Size {
                width: Length::Fixed(0.0),
                height: Length::Fixed(0.0),
            }
        }

        fn layout(
            &self,
            _tree: &mut Tree,
            _renderer: &Renderer,
            _limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(Size::ZERO)
        }

        fn draw(
            &self,
            _tree: &Tree,
            _renderer: &mut Renderer,
            _theme: &Theme,
            _style: &renderer::Style,
            _layout: Layout<'_>,
            _cursor: mouse::Cursor,
            _viewport: &Rectangle,
        ) {
        }
    }

    element.map(T::into).unwrap_or_else(|| Element::new(Void))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
