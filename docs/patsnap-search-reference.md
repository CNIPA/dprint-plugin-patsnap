# 智慧芽(Patsnap)搜索参考手册

> 来源: https://analytics.zhihuiya.com/search_helper
>
> 本文档从智慧芽搜索帮助页面提取，供本地离线参考。

---

## 一、可搜索字段

### 1.1 号码 & 文本

| 字段缩写 | 字段名称 | 类型 | 示例和备注 |
|---------|---------|------|---------|
| PN | 公开(公告)号 | 号码 | PN:US1234567 |
| APNO | 申请号 | 号码 | APNO:US10/123456 |
| PRNO | 优先权号 | 号码 | PRNO:JP2013270967 |
| KD | 文献代码 | 号码 | KD:B1 |
| PCT_PN | PCT国际申请公开号 | 号码 | PCT_PN:WO2017161368 |
| PCT_APNO | PCT国际申请申请号 | 号码 | PCT_APNO:US2017/023191 |
| FAM_ID | 简单同族编号 | 号码 | FAM_ID:138860071 |
| IFAM_ID | INPADOC同族编号 | 号码 | IFAM_ID:438860071 |
| EFAM_ID | PatSnap同族编号 | 号码 | EFAM_ID:742326667 |
| TTL | 标题 | 文本 | TTL:汽车 |
| ABST | 摘要 | 文本 | ABST:汽车 |
| CLMS | 权利要求 | 文本 | CLMS:汽车 |
| ICLMS | 独立权利要求 | 文本 | ICLMS:car (仅支持英文语言专利及受理局为CN的专利) |
| DESC | 说明书 | 文本 | DESC:汽车 |
| DESC_F | 技术领域 | 文本 | DESC_F:汽车 (仅支持受理局为CN的专利搜索) |
| DESC_B | 背景技术 | 文本 | DESC_B:汽车 (仅支持受理局为CN的专利搜索) |
| DESC_S | 发明内容 | 文本 | DESC_S:汽车 (仅支持受理局为CN的专利搜索) |
| DESC_D | 附图说明 | 文本 | DESC_D:汽车 (仅支持受理局为CN的专利搜索) |
| DESC_E | 具体实施方式 | 文本 | DESC_E:汽车 (仅支持受理局为CN的专利搜索) |
| PROBLEM_SUM | 技术问题 | 文本 | PROBLEM_SUM:透气性差 (支持搜索有说明书文本的全球专利) |
| METHOD_SUM | 技术手段 | 文本 | METHOD_SUM:汽车 (支持搜索有说明书文本的全球专利) |
| BENEFIT_SUM | 技术功效 | 文本 | BENEFIT_SUM:汽车 (支持搜索有说明书文本的全球专利) |
| PROBLEM_PHR | [标]技术问题短语 | 文本 | PROBLEM_PHR:成本高 (仅支持受理局为CN/US/EP/WO专利搜索) |
| BENEFIT_PHR | [标]技术功效短语 | 文本 | BENEFIT_PHR:降低成本 (仅支持受理局为CN/US/EP/WO专利搜索) |
| TA | 标题/摘要 | 文本 | TA:(汽车 and 座椅) |
| TAC | 标题/摘要/权利要求 | 文本 | TAC:(汽车 and 座椅) |
| TACD | 标题/摘要/权利要求/说明书 | 文本 | TACD:(汽车 and 座椅) |
| MAINF | 主要字段 | 文本 | MAINF:汽车 (包括标题、摘要、权利要求、说明书、公开号、申请号、申请人、发明人和IPC/UPC/LOC分类号。未指定字段时默认搜索范围) |

#### 英文机翻字段

| 字段缩写 | 字段名称 | 示例 |
|---------|---------|------|
| TTL_ENTRANS | 英文机翻标题 | TTL_ENTRANS:car |
| ABST_ENTRANS | 英文机翻摘要 | ABST_ENTRANS:car |
| CLMS_ENTRANS | 英文机翻权利要求 | CLMS_ENTRANS:car |
| ICLMS_ENTRANS | 英文机翻独立权利要求 | ICLMS_ENTRANS:car |
| DESC_ENTRANS | 英文机翻说明书 | DESC_ENTRANS:car |

#### 中文机翻字段

| 字段缩写 | 字段名称 | 示例 |
|---------|---------|------|
| TTL_CNTRANS | 中文机翻标题 | TTL_CNTRANS:汽车 |
| ABST_CNTRANS | 中文机翻摘要 | ABST_CNTRANS:汽车 |
| CLMS_CNTRANS | 中文机翻权利要求 | CLMS_CNTRANS:汽车 |
| ICLMS_CNTRANS | 中文机翻独立权利要求 | ICLMS_CNTRANS:汽车 |
| DESC_CNTRANS | 中文机翻说明书 | DESC_CNTRANS:汽车 |

#### 原文和翻译字段

| 字段缩写 | 字段名称 | 示例 |
|---------|---------|------|
| TTL_ALL | 标题原文和翻译 | TTL_ALL:汽车 |
| ABST_ALL | 摘要原文和翻译 | ABST_ALL:汽车 |
| CLMS_ALL | 权利要求原文和翻译 | CLMS_ALL:汽车 |
| ICLMS_ALL | 独立权利要求原文和翻译 | ICLMS_ALL:汽车 |
| DESC_ALL | 说明书原文和翻译 | DESC_ALL:汽车 |
| TA_ALL | 标题/摘要原文和翻译 | TA_ALL:汽车 |
| TAC_ALL | 标题/摘要/权利要求原文和翻译 | TAC_ALL:汽车 |
| TACD_ALL | 标题/摘要/权利要求/说明书原文和翻译 | TACD_ALL:汽车 |
| MAINF_ALL | 主要字段原文和翻译 | MAINF_ALL:汽车 |

### 1.2 公司 & 人

| 字段缩写 | 字段名称 | 类型 | 示例和备注 |
|---------|---------|------|---------|
| ALL_AN | [全字段]申请(专利权)人 | 文本 | ALL_AN:清华大学 (联合索引: 包含[标]当前、当前、[标]原始、原始、历史申请人) |
| AN | 原始申请(专利权)人 | 文本 | AN:清华大学 |
| ANC | 当前申请(专利权)人 | 文本 | ANC:清华大学 |
| ANS | [标]原始申请(专利权)人 | 文本 | ANS:清华大学 (经智慧芽标准化处理) |
| ANS_EXACT | 精准[标]原始申请(专利权)人 | 文本 | ANS_EXACT:清华大学 (符号、大小写全匹配) |
| ANCS | [标]当前申请(专利权)人 | 文本 | ANCS:清华大学 (经智慧芽标准化处理) |
| ANCS_EXACT | 精准[标]当前申请(专利权)人 | 文本 | ANCS_EXACT:清华大学 (符号、大小写全匹配) |
| AN_HIST | 历史申请(专利权)人 | 文本 | AN_HIST:清华大学 (包括转移、变更、质押、担保过程中存在过的) |
| GNAME | 自定义申请人组 | 文本 | GNAME:清华大学 |
| AN_ADD | 原始申请(专利权)人地址 | 文本 | AN_ADD:北京 |
| AN_COUNTRY | 原始申请(专利权)人区域 | 文本 | AN_COUNTRY:CN 或 AN_COUNTRY:中国 |
| AN_PROVINCE | 原始申请(专利权)人州/省 | 文本 | AN_PROVINCE:江苏 或 AN_PROVINCE:32 (仅CN) |
| AN_CITY | 原始申请(专利权)人地市 | 文本 | AN_CITY:南京 (仅CN、US) |
| AN_DISTRICT | 原始申请(专利权)人区县 | 文本 | AN_DISTRICT:鼓楼区 (仅CN、US) |
| ANC_ADD | 当前申请(专利权)人地址 | 文本 | ANC_ADD:北京 |
| ANC_COUNTRY | 当前申请(专利权)人区域 | 文本 | ANC_COUNTRY:CN 或 ANC_COUNTRY:中国 |
| ANC_PROVINCE | 当前申请(专利权)人州/省 | 文本 | ANC_PROVINCE:江苏 或 ANC_PROVINCE:32 (仅CN) |
| ANC_CITY | 当前申请(专利权)人地市 | 文本 | ANC_CITY:南京 (仅CN、US) |
| ANC_DISTRICT | 当前申请(专利权)人区县 | 文本 | ANC_DISTRICT:鼓楼区 (仅CN、US) |
| F_AN | 第一原始申请(专利权)人 | 文本 | F_AN:清华大学 |
| F_ANC | 第一当前申请(专利权)人 | 文本 | F_ANC:清华大学 |
| ANS_TYPE | [标]原始申请(专利权)人类型 | 文本 | ANS_TYPE:ACADEMY (ACADEMY/COMPANY/GOVERNMENT/PERSON/HOSPITAL/BANK) |
| ANCS_TYPE | [标]当前申请(专利权)人类型 | 文本 | ANCS_TYPE:ACADEMY |
| IN | 发明人 | 文本 | IN:李书福 |
| INC | 当前发明人 | 文本 | INC:李书福 (仅CN) |
| IN_EXACT | 精准发明人 | 文本 | IN_EXACT:李书福 (符号、大小写全匹配) |
| IN_ADDRESS | 发明人地址 | 文本 | IN_ADDRESS:北京 |
| F_IN | 第一发明人 | 文本 | F_IN:李书福 |
| AT | 代理人 | 文本 | AT:吴观乐 |
| AT_C | 当前代理人 | 文本 | AT_C:"GOUDREAU GAGE DUBUC" (仅CA) |
| ATC | 代理机构 | 文本 | ATC:柳沈 |
| ATCS | [标]代理机构 | 文本 | ATCS:柳沈 (仅CN/US/EP/WO) |
| ATCC | 当前代理机构 | 文本 | ATCC:柳沈 (仅CN) |
| ATC_VALUE | 委托代理机构专利 | 数字 | ATC_VALUE:1 (0:无 1:有) |
| PE | 审查员 | 文本 | PE:王剑 |
| AE | 助理审查员 | 文本 | AE:"SARTORI, MICHAEL A." (仅US) |
| AUTHORITY | 受理局 | 文本 | AUTHORITY:US |
| PRIORITY_COUNTRY | 优先权国家/地区 | 文本 | PRIORITY_COUNTRY:US |
| EPDS | EP指定国家/地区 | 文本 | EPDS:DE |
| AN_EN | 工商英文名 | 文本 | AN_EN:Xiaomi Technology (仅中国工商) |
| BI_USCC | 工商统一社会信用代码 | 文本 | BI_USCC:914110007324785523 (仅中国工商) |
| BI_RN | 工商注册号 | 号码 | BI_RN:411000100003058 (仅中国工商) |
| BI_ADD | 工商注册地址 | 文本 | BI_ADD:北京 (仅中国工商) |
| BI_ET | 工商企业类型 | 文本 | BI_ET:有限责任公司 (仅中国工商) |
| BI_ED | 工商成立日期 | 号码 | BI_ED:20210101 (仅中国工商) |
| BI_RS | 工商登记状态 | 文本 | BI_RS:存续 (仅中国工商) |
| LC_CODE | 工商上市公司代码 | 号码 | LC_CODE:002594 (仅中国工商) |

### 1.3 日期 & 分类

#### 日期字段

| 字段缩写 | 字段名称 | 示例 |
|---------|---------|------|
| APD | 申请日 | APD:[20010101 TO 20011231] |
| APD_Y | 申请年 | APD_Y:2001 |
| APD_YM | 申请年月 | APD_YM:200405 |
| PBD | 公开日 | PBD:[20010101 TO 20011231] |
| F_PBD | 首次公开日 | F_PBD:[20150101 TO 20151231] |
| PBD_Y | 公开年 | PBD_Y:2011 |
| PBD_YM | 公开年月 | PBD_YM:201403 |
| EFAM_EPBD | Patsnap同族最早公开日 | EFAM_EPBD:[20120101 TO 20231231] |
| EFAM_EPBY | Patsnap同族最早公开年 | EFAM_EPBY:2012 |
| EFAM_EPBYM | Patsnap同族最早公开年月 | EFAM_EPBYM:201201 |
| EFAM_EPRD | Patsnap同族最早优先权日 | EFAM_EPRD:[20120101 TO 20231231] |
| ISD | 授权日 | ISD:[20010101 TO 20011231] |
| EXPD | 失效日 | EXPD:[20180101 TO 20181231] (部分国家) |
| EXAMINE_DATE | 实质审查生效日 | EXAMINE_DATE:[20150101 TO 20151231] (部分国家) |
| PCTENTRY_DATE | PCT进入国家阶段日 | PCTENTRY_DATE:[20010101 TO 20011231] |
| LEGAL_STATUS_DATE | 法律状态更新日 | LEGAL_STATUS_DATE:[20150101 TO 20151231] |
| PRIORITY_DATE | 优先权日 | PRIORITY_DATE:[20010101 TO 20011231] |
| E_PRIORITY_DATE | 最早优先权日 | E_PRIORITY_DATE:[20010101 TO 20011231] |
| EXDT | 预估到期日 | EXDT:[20160101 TO 20181231] |

#### 分类号字段

| 字段缩写 | 字段名称 | 示例 |
|---------|---------|------|
| CLASS | 联合分类号 | CLASS:A21B3/04 (包括IPC/CPC/LOC/FI/FTERM/UPC) |
| IPC | IPC分类号 | IPC:A61K 或 IPC:[A21B3/04 TO A21B3/16] |
| IPC_SECTION | IPC分类号部 | IPC_SECTION:H |
| IPC_CLASS | IPC分类号大类 | IPC_CLASS:H04 |
| IPC_SUB_CLASS | IPC分类号小类 | IPC_SUB_CLASS:H04W |
| IPC_GROUP | IPC分类号大组 | IPC_GROUP:A21B3 |
| IPC_SUB_GROUP | IPC分类号小组 | IPC_SUB_GROUP:A21B3/04 |
| MIPC | IPC主分类号 | MIPC:A61K 或 MIPC:[A21B3/04 TO A21B3/16] |
| MIPC_SECTION | IPC主分类号部 | MIPC_SECTION:A |
| MIPC_CLASS | IPC主分类号大类 | MIPC_CLASS:A61 |
| MIPC_SUB_CLASS | IPC主分类号小类 | MIPC_SUB_CLASS:A61K |
| IPC_CPC | IPC/CPC | IPC_CPC:B01B1/00 (同时搜索IPC和CPC) |
| CPC | CPC分类号 | CPC:G10L15/193 或 CPC:[A21B3/04 TO A21B3/16] |
| CPC_ALL | 官方/预测CPC分类号 | CPC_ALL:G10L15/193 (包含智慧芽预测CPC) |
| CPC_SECTION | CPC分类号部 | CPC_SECTION:G |
| CPC_CLASS | CPC分类号大类 | CPC_CLASS:G10 |
| CPC_SUB_CLASS | CPC分类号小类 | CPC_SUB_CLASS:G10L |
| CPC_GROUP | CPC分类号大组 | CPC_GROUP:G10L15 |
| CPC_SUB_GROUP | CPC分类号小组 | CPC_SUB_GROUP:G10L15/193 |
| MCPC | CPC主分类号 | MCPC:H02J7/007 |
| GBC | 国民经济行业分类号 | GBC:A0119 |
| GBC_SECTION | 国民经济行业分类门类 | GBC_SECTION:A |
| LOC | LOC分类号 | LOC:01-01 (仅外观专利) |
| UPC | UPC分类号 | UPC:530/388.2 (仅US) |
| FI | FI分类号 | FI:C12N5/00.A (仅JP) |
| FTERM | F-TERM分类号 | FTERM:4B024/HA11 (仅JP) |
| IPC_LOW | IPC分类号下位组 | IPC_LOW:A01B1/02 (当前分类及子分类) |
| MIPC_LOW | IPC主分类号下位组 | MIPC_LOW:A01B1/02 |
| CPC_LOW | CPC分类号下位组 | CPC_LOW:A01B1/02 |
| UPC_LOW | UPC分类号下位组 | UPC_LOW:126/567 (仅US) |
| FI_LOW | FI分类号下位组 | FI_LOW:A01B1/02 (仅JP) |
| FTERM_LOW | F-TERM分类号下位组 | FTERM_LOW:2B002/AA01 (仅JP) |
| ADC | 应用领域分类 | ADC:"发动机制造" |
| TTC | 技术主题分类 | TTC:"麦克风" |
| SEIC | 战略新兴产业分类(主分类) | SEIC:"人工智能" |
| SEIC_ALL | 战略新兴产业分类 | SEIC_ALL:"人工智能" |

### 1.4 引用 & 同族

| 字段缩写 | 字段名称 | 类型 | 示例和备注 |
|---------|---------|------|---------|
| B_CITES | 引用专利 | 号码 | B_CITES:US6394621B1 |
| F_CITES | 被引用专利 | 号码 | F_CITES:US6394621B1 |
| BF_CITES | 引用或被引用专利 | 号码 | BF_CITES:US6394621B1 |
| B_CITES_COUNT | 引用专利数量 | 数字 | B_CITES_COUNT:[10 TO 20] |
| F_CITES_COUNT | 被引用专利数量 | 数字 | F_CITES_COUNT:[10 TO 20] |
| F_CITES_ANC | 引用当前申请人 | 文本 | F_CITES_ANC:清华大学 |
| B_CITES_ANC | 当前申请人引用 | 文本 | B_CITES_ANC:清华大学 |
| CITE_CATEGORY | 引用类别 | 文本 | CITE_CATEGORY:X (X:特别相关 Y:结合否定创造性 A:一般 D:申请人引证 E:在先文献 L:优先权怀疑 R:同样发明 T:在后文件 101/102/103:美国条款) |
| FAM | 简单同族 | 号码 | FAM:WO2012001234A1 |
| IFAM | INPADOC同族 | 号码 | IFAM:WO2012001234A1 |
| EFAM | PatSnap同族 | 号码 | EFAM:WO2012001234A1 |
| FAM_COUNT | 简单同族专利申请数量 | 数字 | FAM_COUNT:[10 TO 20] |
| IFAM_COUNT | INPADOC同族专利申请数量 | 数字 | IFAM_COUNT:[10 TO 20] |
| EFAM_COUNT | Patsnap同族专利申请数量 | 数字 | EFAM_COUNT:[10 TO 20] |
| EFAM_COUNTRY | PatSnap同族国家/地区 | 文本 | EFAM_COUNTRY:CN |
| EFAM_EPB_COUNTRY | Patsnap同族最早公开国家/地区 | 文本 | EFAM_EPB_COUNTRY:CN |
| FAM_COUNTRY | 简单同族国家/地区 | 文本 | FAM_COUNTRY:CN |
| FAM_COUNTRY_COUNT | 简单同族国家/地区数量 | 数字 | FAM_COUNTRY_COUNT:[1 TO 3] |
| IFAM_COUNTRY_COUNT | INPADOC同族国家/地区数量 | 数字 | IFAM_COUNTRY_COUNT:[1 TO 3] |
| EFAM_COUNTRY_COUNT | PatSnap同族国家/地区数量 | 数字 | EFAM_COUNTRY_COUNT:[1 TO 3] |
| EPDS_COUNT | EP指定国家/地区数量 | 数字 | EPDS_COUNT:[1 TO 3] |

### 1.5 专利状态 & 质量

| 字段缩写 | 字段名称 | 类型 | 示例和备注 |
|---------|---------|------|---------|
| LEGAL_STATUS | 法律状态 | 数字 | LEGAL_STATUS:3 |
| LEGAL_EVENT | 法律事件 | 数字 | LEGAL_EVENT:61 (61:权利转移 62:许可 63:质押 64:信托 65:异议 66:复审 69:海关备案 70:诉讼 71:保全 72:无效程序 73:口头审理 74:国防解密 75:一案双申 76:期限延长) |
| SIMPLE_LEGAL_STATUS | 简单法律状态 | 数字 | SIMPLE_LEGAL_STATUS:1 (0:失效 1:有效 2:审中 220:PCT指定期满 221:PCT指定期内 999:未确认) |
| EFAM_STATUS | PatSnap同族状态 | 数字 | EFAM_STATUS:1 (0:失效 1:有效 2:审中 999:未确认) |
| UP_STATUS | 欧洲统一法院状态 | 数字 | UP_STATUS:1 (0:未选择退出 1:选择退出 2:退出后撤回 3:UP专利已注册) |
| ENTRY_COUNTRY_LS | 进入国家/地区法律状态 | 数字 | ENTRY_COUNTRY_LS:3 |
| ENTRY_COUNTRY_SLS | 进入国家/地区简单法律状态 | 数字 | ENTRY_COUNTRY_SLS:1 (0:失效 1:有效 2:审中 999:未确认) |
| EPDS_LS | EP指定国家/地区法律状态 | 数字 | EPDS_LS:3 (3:授权 14:全部撤销 15:期限届满 16:未缴年费 21:权利恢复 22:权利终止 23:部分无效 30:放弃 19:视为放弃 20:主动放弃 25:未指定类型) |
| EPDS_SLS | EP指定国家/地区简单法律状态 | 数字 | EPDS_SLS:1 (0:失效 1:有效 999:未确认) |
| PV | 专利价值 | 数字 | PV:[10000 TO 50000] |
| PAGE_COUNT | 文献页数 | 数字 | PAGE_COUNT:[5 TO 10] (部分国家，不含外观) |
| CLAIM_COUNT | 权利要求数 | 数字 | CLAIM_COUNT:[10 TO 20] |
| FCLMS_WORDCOUNT | 第一权利要求字数 | 数字 | FCLMS_WORDCOUNT:[100 TO 300] (仅中英文原文) |
| AN_COUNT | 原始申请(专利权)人数量 | 数字 | AN_COUNT:[2 TO 3] |
| ANC_COUNT | 当前申请(专利权)人数量 | 数字 | ANC_COUNT:[2 TO 3] |
| IN_COUNT | 发明人数量 | 数字 | IN_COUNT:[2 TO 3] |
| CPC_COUNT | CPC分类数量 | 数字 | CPC_COUNT:[2 TO 3] |
| IPC_COUNT | IPC分类数量 | 数字 | IPC_COUNT:[2 TO 3] |
| GOV | 政府利益 | 文本 | GOV:1 (仅US) |
| EXAMINE_PERIOD | 审查时长 | 数字 | EXAMINE_PERIOD:[5 TO 10] (单位:月，部分国家) |
| PATENT_TYPE | 专利类型 | 文本 | PATENT_TYPE:D (A:发明申请 B:授权发明 U:实用新型 D:外观设计) |
| PCTENTRY_TYPE | PCT路径进入专利 | 文本 | PCTENTRY_TYPE:1 |
| EP_ENTRY | EPO路径进入专利 | 文本 | EP_ENTRY:1 |
| ENTRY_COUNTRY | 进入国家/地区 | 文本 | ENTRY_COUNTRY:DE |
| SEP | 所有标准专利 | 数字 | SEP:1 |
| SEP_NUMBER | 标准号 | 文本 | SEP_NUMBER:"TS 138 211" |
| SEP_TITLE | SEP标准标题 | 文本 | SEP_TITLE:E-UTRA (仅英文) |
| SEP_SOURCE | SEP数据源 | 文本 | SEP_SOURCE:ETSI (ETSI/IEC/IEEE/ISO/ANSI/CEN/ITU) |
| SEP_PROJECT | 标准-项目 | 文本 | SEP_PROJECT:5G (仅英文) |
| SEP_DECLARANT | SEP标准持有者 | 文本 | SEP_DECLARANT:huawei (仅英文) |
| AWARD_NAME | 奖励名称 | 文本 | AWARD_NAME:(中国专利奖) (仅CN和欧洲) |
| AWARD_LEVEL | 奖励等级 | 文本 | AWARD_LEVEL:(金奖) (仅CN) |
| AWARD_SESSION | 奖励届次 | 文本 | AWARD_SESSION:(第一届) (仅CN和欧洲) |
| SUB_CASE | 分案 | 号码 | SUB_CASE:(US10/771529) |
| PRIORITY_COUNTRY_COUNT | 优先权国家/地区个数 | 数字 | PRIORITY_COUNTRY_COUNT:[10 TO 20] |
| PRIORITY_EMPTY | 所有无优先权专利 | 数字 | PRIORITY_EMPTY:1 (0:有优先权 1:无优先权) |

### 1.6 诉讼

| 字段缩写 | 字段名称 | 类型 | 示例和备注 |
|---------|---------|------|---------|
| LITIGATION | 专利诉讼 | 数字 | LITIGATION:1 (部分国家) |
| CASENO | 案件号 | 文本 | CASENO:("(2012)浙杭知初字第425号") |
| COURT | 审理法院 | 文本 | COURT:(浙江省杭州市中级人民法院) |
| JUDGE | 审判员 | 文本 | JUDGE:(王江桥) |
| CHIEF_JUDGE | 审判长 | 文本 | CHIEF_JUDGE:张艳培 |
| PLAINTIFF | 原告 | 文本 | PLAINTIFF:(华为) |
| DEFENDANT | 被告 | 文本 | DEFENDANT:(中兴) |
| FILING_DATE | 立案日期 | 日期 | FILING_DATE:[20150101 TO 20150630] |
| VERDICT_DATE | 裁判日期 | 日期 | VERDICT_DATE:[20150101 TO 20151231] |
| HEARING_DATE | 听证日期 | 日期 | HEARING_DATE:[20150101 TO 20151231] |
| TRIAL_GRADE | 审理程序 | 文本 | TRIAL_GRADE:(一审) |
| CASE_NATURE | 案件性质 | 文本 | CASE_NATURE:(民事案件) |
| CASE_LEVEL | 案例级别 | 文本 | CASE_LEVEL:(1) (仅CN, 1:典型 2:普通) |
| CASE_REGION | 审理地域 | 文本 | CASE_REGION:(江苏) |
| CASE_TITLE | 案件标题 | 文本 | CASE_TITLE:(华为) |
| CASE_FULL_TEXT | 案件全文 | 文本 | CASE_FULL_TEXT:(照明) |
| LIT_CLOSEDT | 结案日期 | 日期 | LIT_CLOSEDT:[20150101 TO 20151231] |
| OUTCOME_JUDGEMENT | 胜诉方 | 文本 | OUTCOME_JUDGEMENT:(原告) |
| OUTCOME_STATUS | 案件状态 | 文本 | OUTCOME_STATUS:("Closed (closed)") |
| LITIGATION_COUNT | 诉讼次数 | 数字 | LITIGATION_COUNT:2 |
| CASE_DOC_TYPE | 文书类型 | 文本 | CASE_DOC_TYPE:2 (仅CN, 1:判决书 2:裁定书 3:调解书 4:决定书 5:其他) |
| COURT_GRADE | 法院级别 | 文本 | COURT_GRADE:中级法院 (仅CN) |
| VERDICT | 判决结果 | 文本 | VERDICT:(驳回) (暂不支持US) |
| PARTY | 当事人 | 文本 | PARTY:(华为) (含原告、被告及第三方) |
| PARTY_AGENT | 当事人委托代理人 | 文本 | PARTY_AGENT:(王佩佩) |
| PARTY_LAWFIRM | 当事人委托律所 | 文本 | PARTY_LAWFIRM:(广东非凡律师事务所) |
| AMOUNT_PLAINTIFF | 申请赔偿总额 | 数字 | AMOUNT_PLAINTIFF:[10000 TO 50000] (仅CN) |
| DAMAGES_AMOUNT | 判赔总额 | 数字 | DAMAGES_AMOUNT:[10000 TO 50000] (仅CN) |
| FILING_YEAR | 立案年份 | 日期 | FILING_YEAR:[2011 TO 2012] |
| LITIGATION_PRODUCT | 涉及产品 | 文本 | LITIGATION_PRODUCT:(真空运动瓶) |

### 1.7 许可

| 字段缩写 | 字段名称 | 类型 | 示例和备注 |
|---------|---------|------|---------|
| LICENSE | 所有发生许可的专利 | 数字 | LICENSE:1 |
| LICENSOR | 许可人 | 文本 | LICENSOR:(三星) |
| LICENSEE | 被许可人 | 文本 | LICENSEE:(小米) |
| LICNO | 许可合同备案号 | 文本 | LICNO:2014510000101 |
| EXCLUSIVITY | 许可排他性 | 文本 | EXCLUSIVITY:(Exclusive) |
| LIC_EFDT | 许可生效日 | 日期 | LIC_EFDT:[20160101 TO 20160501] |
| LICENSE_COUNT | 许可次数 | 数字 | LICENSE_COUNT:2 |

### 1.8 权利转移

| 字段缩写 | 字段名称 | 类型 | 示例和备注 |
|---------|---------|------|---------|
| TRANSFER | 所有发生转让的专利 | 数字 | TRANSFER:1 |
| TRANSFER_BEFORE | 转让人 | 文本 | TRANSFER_BEFORE:(英特尔) |
| TRANSFER_AFTER | 受让人 | 文本 | TRANSFER_AFTER:(华为) |
| TRANS_EFDT | 权利转移生效日 | 日期 | TRANS_EFDT:[20180701 TO 20181231] |

### 1.9 复审无效

| 字段缩写 | 字段名称 | 类型 | 示例和备注 |
|---------|---------|------|---------|
| REEXAMINVALID | 所有发生复审/无效的专利 | 数字 | REEXAMINVALID:1 |
| RI_APPLICANT | 复审/无效请求人 | 文本 | RI_APPLICANT:彭宇轴 |
| RIDN | 决定号 | 文本 | RIDN:(FS1112) |
| RIIN | 委内编号 | 文本 | RIIN:(F12964) |
| RIDDT | 决定/发文日 | 日期 | RIDDT:[20160101 TO 20160501] |
| RIDTP | 决定类型 | 文本 | RIDTP:(FS) |
| RID | 决定 | 文本 | RID:(全部无效) |
| RIDP | 决定要点 | 文本 | RIDP:(技术方案) |
| RIDSM | 案由 | 文本 | RIDSM:(不符合专利法第22条第3款的规定) |
| RILGS | 法律依据 | 文本 | RILGS:(专利法第22条第3款) |
| RI_FULL_TEXT | 复审/无效全文 | 文本 | RI_FULL_TEXT:(技术问题) (仅CN) |
| INVALID_COUNT | 无效次数 | 数字 | INVALID_COUNT:2 |

### 1.10 质押

| 字段缩写 | 字段名称 | 类型 | 示例和备注 |
|---------|---------|------|---------|
| PLEDGE | 专利质押 | 数字 | PLEDGE:1 |
| PLEDGOR | 质押人 | 文本 | PLEDGOR:auto (仅CN质押、US担保) |
| PLEDGEE | 质权人 | 文本 | PLEDGEE:(SILICON VALLEY BANK) (仅CN质押、US担保) |
| PLEDGENO | 质押登记号 | 文本 | PLEDGENO:2015990000410 (仅CN质押、US担保) |
| PLE_EFDT | 质押生效日 | 日期 | PLE_EFDT:[20160101 TO 20160501] (仅CN质押、US担保) |
| PLE_STAGE | 质押备案阶段 | 文本 | PLE_STAGE:1 (仅CN) |
| PLEDGE_COUNT | 质押次数 | 数字 | PLEDGE_COUNT:2 |

### 1.11 工作空间

| 字段缩写 | 字段名称 | 类型 | 示例和备注 |
|---------|---------|------|---------|
| MWS | 我的工作空间 | 文本 | MWS:\|工作空间A\文件夹B\| |
| CWS | 企业工作空间 | 文本 | CWS:\|工作空间A\文件夹B\| |
| CCF_ | 企业自定义字段 | 文本 | CCF_PR:"标引值A" |

---

## 二、搜索语法

### 2.1 逻辑运算符

| 符号 | 作用 | 示例 |
|-----|------|------|
| AND | 要求符号两边的关键词同时存在，对位置顺序没有要求；关键词之间有空格且无运算符时默认AND | `太阳能 AND 电池` / `TTL:纳米 AND AN:清华大学` |
| OR | 要求符号两边的关键词至少出现任意一个 | `发动机 OR 引擎` / `TTL:非晶硅电池 OR IPC:H01L31/075` |
| NOT | 排除符号后面的关键词。优先级: NOT > AND > OR，可用()提高优先级 | `电视 NOT 等离子` / `AN:华为科技 NOT ABST:手机` |
| GAND | 将搜索限制到事件组中的特定成员。事件组包括: 奖励(AWARD_NAME/LEVEL/SESSION)、EP指定国(EPDS/EPDS_SLS/EPDS_LS)、引用(CITE相关/CITE_CATEGORY)、自定义字段(CCF/CWS) | `EPDS:DE GAND EPDS_SLS:1` |

### 2.2 通配符

| 符号 | 作用 | 示例 |
|-----|------|------|
| * | 代表0个或多个字符。一个关键词中最多两个*，不能与其他通配符同时用。中间/末尾需前保留2+字符，词首限TTL/ABST/CLMS/DESC相关字段且仅英文且后需3+字符。两个*中间需3+字符。不支持引号中使用 | `*otor` / `*oto*` / `electr*` / `EP200*B2` / `小*车` |
| ? | 代表1个字符，可用多个表示指定数量。必须连在一起。词首限TTL/ABST/CLMS/DESC相关字段且仅英文。不支持引号中使用 | `?otor` / `gra???ne` / `US7654???` / `小??车` |
| # | 代表0个或1个字符。一个关键词中只能用一个，不能与其他通配符同时用。词首限TTL/ABST/CLMS/DESC相关字段且仅英文。不支持引号中使用 | `#otor` / `m#tor` / `moto#` / `小#车` |

### 2.3 位置符(邻近检索)

| 符号 | 作用 | 示例 |
|-----|------|------|
| $Wn | 两个关键词之间间隔不超过n个单词或汉字(0<=n<=99)，无位置顺序要求。优先级高于逻辑运算符 | `data $W2 line` / `ABST:(太阳能 $W2 电池 $W2 硅)` |
| $PREn | 两个关键词之间间隔不超过n个单词或汉字(0<=n<=99)，限定位置顺序。优先级高于逻辑运算符 | `data $PRE2 line` / `ABST:(太阳能 $PRE2 电池 $PRE2 硅)` |
| $WS | 两个关键词之间间隔不超过99个单词或汉字。优先级高于逻辑运算符 | `显示 $WS 高清` / `ABST:(显示 $WS 屏幕 $WS 高清)` |
| $SEN | 两个或多个关键词出现在同一个句子中。优先级高于逻辑运算符 | `CLMS:(data $SEN line)` / `DESC:(太阳能 $SEN 电池 $SEN 汽车)` |
| $PARA | 两个或多个关键词出现在同一个段落中。优先级高于逻辑运算符 | `DESC:(data $PARA line)` / `DESC:(太阳能 $PARA 电池 $PARA 汽车)` |

### 2.4 频率运算符

| 符号 | 作用 | 示例 |
|-----|------|------|
| $FREQn | 关键词在指定字段中出现至少n次。仅限TTL/ABST/CLMS/DESC及其组合字段，不支持机翻。n最大值50。仅限单个关键词或短语，仅限公开库 | `汽车 $FREQ2` / `TTL:("car" $FREQ2) AND ABST:("engine")` |

### 2.5 其他符号

| 符号 | 作用 | 示例 |
|-----|------|------|
| () | 将括号内的逻辑优先运算 | `汽车 AND (发动机 OR 引擎)` |
| [] | 范围查询，限定日期、数字、分类号的起止范围 | `PBD:[20010101 TO 20101231]` / `IPC:[H01L31/0203 TO H01L31/042]` |
| " " | 短语查询，固定引号内关键词的位置顺序 | `"electric vehicle"` |
| _ | 可选分隔符，匹配空格、中线"-"和无空格。一个关键词中只能用一个(引号中或多个_或与通配符一起时作为普通字符) | `T_shirt` 可匹配 `T shirt`、`T-shirt`、`Tshirt` |
| 截词 | 开启后自动扩展英文单复数及时态。仅作用于英文关键词，人名字段不生效。使用通配符时截词对该关键词不生效 | 开启: `TTL:come` 匹配 come/comes/came/coming 等 |
| TREE@ | 搜索PatSnap公司树中一个名称及其下属公司的所有专利。支持ANS/ANCS/ALL_AN字段，名称必须是公司树节点全名 | `ANCS:(TREE@"拜耳股份公司")` |
| 特殊字符 | 作为单独字符支持检索: - / ℃ ℉ % +- ° (TM) (R) mg/l @ | `ABST:(85℃)` |
