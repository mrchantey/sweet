use super::ParseResult;

pub trait HydrateClient {
	fn hydrate() -> ParseResult<()>;
}
