# iced-optional-element-shim
Allowing iced 0.13.1 and prior versions to use optional elements in rows and columns

## Usage

```
use iced::widget::{text, Text};
use iced_optional_element_shim::to_elem;


fn view(&self) -> Element<'a, Message> {
    column!(
        text("Example header"),

        if true {
            to_elem(Some(text("this element will display")))
        } else {
            to_elem::<Message, Text>(None)
        },

        if false {
            to_elem(Some(text("this element will not display")))
        } else {
            to_elem::<Message, Text>(None)
        },

        text("footer"),
    )
}
```