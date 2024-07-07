// mod rt;
// pub mod server;

// pub fn version_major_minor() -> String {
//     rt::with_exports(|exports| exports.get::<_, String>("versionMajorMinor").unwrap())
// }
// #[cfg(test)]
// #[test]
// fn test_version_major_minor() {
//     println!("version_major_minor={:?}", version_major_minor());
//     assert_eq!(version_major_minor(), "5.5");
// }

// pub fn version() -> String {
//     rt::with_exports(|exports| exports.get::<_, String>("version").unwrap())
// }
// #[cfg(test)]
// #[test]
// fn test_version() {
//     println!("version={:?}", version());
// }

// // TODO: Derive all the things u32 does.
// #[derive(Debug)]
// pub struct SyntaxKind(pub u32);

// #[allow(non_upper_case_globals)]
// impl SyntaxKind {
//     pub const Unknown: SyntaxKind = SyntaxKind(0);
//     pub const EndOfFileToken: SyntaxKind = SyntaxKind(1);
//     pub const SingleLineCommentTrivia: SyntaxKind = SyntaxKind(2);
//     pub const MultiLineCommentTrivia: SyntaxKind = SyntaxKind(3);
// }

// // TODO: Derive into* from* for Rust<->QuickJS conversions.

// pub fn token_to_string(token: SyntaxKind) -> Option<String> {
//     rt::with_exports(|exports| {
//         exports
//             .get::<_, rquickjs::Function>("tokenToString")
//             .unwrap()
//             .call((token.0,))
//             .unwrap()
//     })
// }
// #[cfg(test)]
// #[test]
// fn test_token_to_string() {
//     println!("token_to_string(0)={:?}", token_to_string(SyntaxKind(0)));
// }

// pub fn is_white_space_like(ch: u32) -> bool {
//     rt::with_exports(|exports| {
//         exports
//             .get::<_, rquickjs::Function>("isWhiteSpaceLike")
//             .unwrap()
//             .call((ch,))
//             .unwrap()
//     })
// }
// #[cfg(test)]
// #[test]
// fn test_is_white_space_like() {
//     println!("is_white_space_like(' ' as u32)={:?}", is_white_space_like(' ' as u32));
// }

// pub fn is_line_break(ch: u32) -> bool {
//     rt::with_exports(|exports| {
//         exports
//             .get::<_, rquickjs::Function>("isLineBreak")
//             .unwrap()
//             .call((ch,))
//             .unwrap()
//     })
// }
// #[cfg(test)]
// #[test]
// fn test_is_line_break() {
//     println!("is_line_break('\\n' as u32)={:?}", is_line_break('\n' as u32));
// }

// pub fn could_start_trivia(text: impl AsRef<str>, pos: usize) -> bool {
//     rt::with_exports(|exports| {
//         exports
//             .get::<_, rquickjs::Function>("couldStartTrivia")
//             .unwrap()
//             .call((text.as_ref(), pos))
//             .unwrap()
//     })
// }
// #[cfg(test)]
// #[test]
// fn test_could_start_trivia() {
//     println!("could_start_trivia(\"//\", 0)={:?}", could_start_trivia("//", 0));
// }

