use internal;
use self_tokenize_macro::SelfTokenize;
use self_tokenize_trait::ToCustomTokens;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, SelfTokenize)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum PrintOptions {
	Layout = 1,
	Style = 2,
	Children = 4,
}

impl From<PrintOptions> for internal::YGPrintOptions {
	fn from(p: PrintOptions) -> internal::YGPrintOptions {
		match p {
			PrintOptions::Layout => internal::YGPrintOptions::YGPrintOptionsLayout,
			PrintOptions::Style => internal::YGPrintOptions::YGPrintOptionsStyle,
			PrintOptions::Children => internal::YGPrintOptions::YGPrintOptionsChildren,
		}
	}
}
