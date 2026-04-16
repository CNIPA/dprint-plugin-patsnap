/// Complete lookup table of all known Patsnap searchable field codes.
/// Extracted from the official Patsnap search helper documentation.
/// All comparisons should be case-insensitive.

const FIELD_CODES: &[&str] = &[
    // === 号码&文本 ===
    "PN", "APNO", "PRNO", "KD", "PCT_PN", "PCT_APNO",
    "FAM_ID", "IFAM_ID", "EFAM_ID",
    "TTL", "ABST", "CLMS", "ICLMS", "DESC",
    "DESC_F", "DESC_B", "DESC_S", "DESC_D", "DESC_E",
    "PROBLEM_SUM", "METHOD_SUM", "BENEFIT_SUM",
    "PROBLEM_PHR", "BENEFIT_PHR",
    "TA", "TAC", "TACD", "MAINF",
    "TTL_ENTRANS", "ABST_ENTRANS", "CLMS_ENTRANS", "ICLMS_ENTRANS", "DESC_ENTRANS",
    "TTL_CNTRANS", "ABST_CNTRANS", "CLMS_CNTRANS", "ICLMS_CNTRANS", "DESC_CNTRANS",
    "TTL_ALL", "ABST_ALL", "CLMS_ALL", "ICLMS_ALL", "DESC_ALL",
    "TA_ALL", "TAC_ALL", "TACD_ALL", "MAINF_ALL",

    // === 公司&人 ===
    "ALL_AN", "AN", "ANC", "ANS", "ANS_EXACT", "ANCS", "ANCS_EXACT",
    "AN_HIST", "GNAME",
    "AN_ADD", "AN_COUNTRY", "AN_PROVINCE", "AN_CITY", "AN_DISTRICT",
    "ANC_ADD", "ANC_COUNTRY", "ANC_PROVINCE", "ANC_CITY", "ANC_DISTRICT",
    "F_AN", "F_ANC", "ANS_TYPE", "ANCS_TYPE",
    "IN", "INC", "IN_EXACT", "IN_ADDRESS", "F_IN",
    "AT", "AT_C", "ATC", "ATCS", "ATCC", "ATC_VALUE",
    "PE", "AE",
    "AUTHORITY", "PRIORITY_COUNTRY", "EPDS",
    "AN_EN", "BI_USCC", "BI_RN", "BI_ADD", "BI_ET", "BI_ED", "BI_RS", "LC_CODE",

    // === 日期&分类 ===
    "APD", "APD_Y", "APD_YM",
    "PBD", "F_PBD", "PBD_Y", "PBD_YM",
    "EFAM_EPBD", "EFAM_EPBY", "EFAM_EPBYM", "EFAM_EPRD",
    "ISD", "EXPD", "EXAMINE_DATE", "PCTENTRY_DATE",
    "LEGAL_STATUS_DATE", "PRIORITY_DATE", "E_PRIORITY_DATE", "EXDT",
    "CLASS",
    "IPC", "IPC_SECTION", "IPC_CLASS", "IPC_SUB_CLASS", "IPC_GROUP", "IPC_SUB_GROUP",
    "MIPC", "MIPC_SECTION", "MIPC_CLASS", "MIPC_SUB_CLASS",
    "IPC_CPC",
    "CPC", "CPC_ALL", "CPC_SECTION", "CPC_CLASS", "CPC_SUB_CLASS", "CPC_GROUP", "CPC_SUB_GROUP",
    "MCPC",
    "GBC", "GBC_SECTION",
    "LOC", "UPC", "FI", "FTERM",
    "IPC_LOW", "MIPC_LOW", "CPC_LOW", "UPC_LOW", "FI_LOW", "FTERM_LOW",
    "ADC", "TTC", "SEIC", "SEIC_ALL",

    // === 引用&同族 ===
    "B_CITES", "F_CITES", "BF_CITES",
    "B_CITES_COUNT", "F_CITES_COUNT",
    "F_CITES_ANC", "B_CITES_ANC", "CITE_CATEGORY",
    "FAM", "IFAM", "EFAM",
    "FAM_COUNT", "IFAM_COUNT", "EFAM_COUNT",
    "EFAM_COUNTRY", "EFAM_EPB_COUNTRY",
    "FAM_COUNTRY", "FAM_COUNTRY_COUNT", "IFAM_COUNTRY_COUNT", "EFAM_COUNTRY_COUNT",
    "EPDS_COUNT",

    // === 专利状态&质量 ===
    "LEGAL_STATUS", "LEGAL_EVENT", "SIMPLE_LEGAL_STATUS",
    "EFAM_STATUS", "UP_STATUS",
    "ENTRY_COUNTRY_LS", "ENTRY_COUNTRY_SLS",
    "EPDS_LS", "EPDS_SLS",
    "PV", "PAGE_COUNT", "CLAIM_COUNT", "FCLMS_WORDCOUNT",
    "AN_COUNT", "ANC_COUNT", "IN_COUNT", "CPC_COUNT", "IPC_COUNT",
    "GOV", "EXAMINE_PERIOD",
    "PATENT_TYPE", "PCTENTRY_TYPE", "EP_ENTRY", "ENTRY_COUNTRY",
    "SEP", "SEP_NUMBER", "SEP_TITLE", "SEP_SOURCE", "SEP_PROJECT", "SEP_DECLARANT",
    "AWARD_NAME", "AWARD_LEVEL", "AWARD_SESSION",
    "SUB_CASE",
    "PRIORITY_COUNTRY_COUNT", "PRIORITY_EMPTY",

    // === 诉讼 ===
    "LITIGATION", "CASENO", "COURT", "JUDGE", "CHIEF_JUDGE",
    "PLAINTIFF", "DEFENDANT",
    "FILING_DATE", "VERDICT_DATE", "HEARING_DATE",
    "TRIAL_GRADE", "CASE_NATURE", "CASE_LEVEL", "CASE_REGION",
    "CASE_TITLE", "CASE_FULL_TEXT",
    "LIT_CLOSEDT", "OUTCOME_JUDGEMENT", "OUTCOME_STATUS",
    "LITIGATION_COUNT", "CASE_DOC_TYPE", "COURT_GRADE",
    "VERDICT", "PARTY", "PARTY_AGENT", "PARTY_LAWFIRM",
    "AMOUNT_PLAINTIFF", "DAMAGES_AMOUNT",
    "FILING_YEAR", "LITIGATION_PRODUCT",

    // === 许可 ===
    "LICENSE", "LICENSOR", "LICENSEE", "LICNO",
    "EXCLUSIVITY", "LIC_EFDT", "LICENSE_COUNT",

    // === 权利转移 ===
    "TRANSFER", "TRANSFER_BEFORE", "TRANSFER_AFTER", "TRANS_EFDT",

    // === 复审无效 ===
    "REEXAMINVALID", "RI_APPLICANT", "RIDN", "RIIN",
    "RIDDT", "RIDTP", "RID", "RIDP", "RIDSM", "RILGS",
    "RI_FULL_TEXT", "INVALID_COUNT",

    // === 质押 ===
    "PLEDGE", "PLEDGOR", "PLEDGEE", "PLEDGENO",
    "PLE_EFDT", "PLE_STAGE", "PLEDGE_COUNT",

    // === 工作空间 ===
    "MWS", "CWS",
    // CCF_ is a prefix: CCF_XXX for custom fields, handled specially.
];

/// Check if a given identifier is a known Patsnap field code (case-insensitive).
/// Also handles the CCF_ prefix for custom enterprise fields.
pub fn is_field_code(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    // CCF_ prefix match for enterprise custom fields
    if upper.starts_with("CCF_") {
        return true;
    }
    FIELD_CODES.iter().any(|&code| code == upper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_fields() {
        assert!(is_field_code("TTL"));
        assert!(is_field_code("ttl"));
        assert!(is_field_code("Ttl"));
        assert!(is_field_code("ABST"));
        assert!(is_field_code("tac"));
        assert!(is_field_code("AN"));
        assert!(is_field_code("IPC"));
        assert!(is_field_code("LEGAL_STATUS"));
        assert!(is_field_code("LITIGATION"));
        assert!(is_field_code("PLEDGE"));
        assert!(is_field_code("MWS"));
    }

    #[test]
    fn ccf_prefix() {
        assert!(is_field_code("CCF_PR"));
        assert!(is_field_code("ccf_custom"));
        assert!(is_field_code("CCF_ANYTHING"));
    }

    #[test]
    fn unknown_fields() {
        assert!(!is_field_code("UNKNOWN"));
        assert!(!is_field_code("hello"));
        assert!(!is_field_code("AND"));
        assert!(!is_field_code("OR"));
    }
}
