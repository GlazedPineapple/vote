impl ::askama::Template for PollsList {
    fn render_into(&self, writer: &mut dyn::std::fmt::Write) -> ::askama::Result<()> {
        include_bytes!("/home/eokovacs/Desktop/git/vote/templates/polls.html");
        writer.write_str("<!DOCTYPE html>\n<html lang=\"en\">\n    <head>\n        <meta charset=\"UTF-8\">\n        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n        <title>All polls</title>\n        <style>\n            td, tr, th, table {\n                border: solid black 2px;\n            }\n        </style>\n    </head>\n    <body>\n        <table>\n            <thead>\n                <tr>\n                    <th>Poll Id</th>\n                    <th>Poll Total</th>\n                    <th>Poll Moderators</th>\n                    <th>Poll Choices</th>\n                </tr>\n            </thead>\n            <tbody>\n                ")?;
        for (poll, _loop_item) in ::askama::helpers::TemplateLoop::new((&self.polls).into_iter()) {
            write!(
                writer,
                "\n                    <tr>\n                        <td>{expr0}</td>\n                        <td>{expr1}</td>\n                        <td>\n                            <ul>\n                                ",
                expr0 = &::askama::MarkupDisplay::new_unsafe(&poll.id.to_string(), ::askama::Html),
                expr1 = &::askama::MarkupDisplay::new_unsafe(&poll.title, ::askama::Html),
            )?;
            for (moderator, _loop_item) in
                ::askama::helpers::TemplateLoop::new((&poll.clone().moderators).into_iter())
            {
                write!(
                    writer,
                    "\n                                    <li>{expr2}</li>\n                                ",
                    expr2 = &::askama::MarkupDisplay::new_unsafe(&moderator, ::askama::Html),
                )?;
            }
            writer.write_str("\n                            </ul>\n                        </td>\n                        <td>\n                            <ul>\n                                ")?;
            for ((uuid, choice), _loop_item) in
                ::askama::helpers::TemplateLoop::new((&poll.clone().choices).into_iter())
            {
                write!(
                    writer,
                    "\n                                    <li>Prez: {expr3}, VP: {expr4}</li>\n                                ",
                    expr3 = &::askama::MarkupDisplay::new_unsafe(&choice.president, ::askama::Html),
                    expr4 = &::askama::MarkupDisplay::new_unsafe(&choice.vice_president, ::askama::Html),
                )?;
            }
            writer.write_str("\n                            </ul>\n                        </td>\n                    </tr>\n                ")?;
        }
        writer.write_str("\n            </tbody>\n        </table>\n    </body>\n</html>")?;
        Ok(())
    }
    fn extension(&self) -> Option<&'static str> {
        Some("html")
    }
    fn size_hint(&self) -> usize {
        58
    }
}
impl ::askama::SizedTemplate for PollsList {
    fn size_hint() -> usize {
        58
    }
    fn extension() -> Option<&'static str> {
        Some("html")
    }
}
impl ::std::fmt::Display for PollsList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::askama::Template::render_into(self, f).map_err(|_| ::std::fmt::Error {})
    }
}
