use internal;
use self_tokenize_macro::SelfTokenize;
use self_tokenize_trait::ToCustomTokens;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, SelfTokenize)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum NodeType {
	Default = 0,
	Text = 1,
}

impl From<NodeType> for internal::YGNodeType {
	fn from(n: NodeType) -> internal::YGNodeType {
		match n {
			NodeType::Default => internal::YGNodeType::YGNodeTypeDefault,
			NodeType::Text => internal::YGNodeType::YGNodeTypeText,
		}
	}
}
