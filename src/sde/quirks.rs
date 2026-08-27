use crate::sde::types::ids;

// !! SDE BUG !!
// These blueprint IDs require a material_id that doesn't exist in the SDE:
// 3924->3927
pub(crate) const BROKEN_MATERIAL_BLUEPRINT_IDS: &[ids::TypeID] = &[3927];

// !! SDE BUG !!
// These blueprint IDs produce a product_id that doesn't exist in the SDE:
// 37398->37236, 37399->37237, 37400->37238, 37401->37239, 37402->37240,
// 37403->37241, 37404->37242, 37405->37243, 37406->37244, 37407->37245,
// 37408->37286, 37409->37247, 37425->37262, 37426->37263, 37427->37264,
// 37428->37265, 37429->37266, 37430->37267, 37441->37278, 37442->37279
pub(crate) const BROKEN_PRODUCT_BLUEPRINT_IDS: &[ids::TypeID] = &[
    37398, 37399, 37400, 37401, 37402, 37403, 37404, 37405, 37406, 37407, 37408, 37409, 37425,
    37426, 37427, 37428, 37429, 37430, 37441, 37442,
];
