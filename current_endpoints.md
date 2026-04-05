# Academy

| Method | Route | Private | Description |
| ------ | ----- | ------- | ----------- |
| POST | /academy/enroll/{courseID} |  | Enroll controller in course. |
| GET | /academy/identifiers |  | Get of list course IDs. |
| GET | /academy/transcript/{cid} |  | Retrieve the Academy transcript for a user. |

# Auth

| Method | Route | Private | Description |
| ------ | ----- | ------- | ----------- |
| GET | /auth/info | Yes | Get information about logged in user. |
| GET | /auth/token | Yes | Get JWT. |
| GET | /auth/token/refresh | Yes | Refresh JWT. |
| GET | /bucket/(facility) |  | Get bucket information. |
| POST | /bucket/(facility) |  | Create bucket. Requires JWT/Session Key |

# Email

| Method | Route | Private | Description |
| ------ | ----- | ------- | ----------- |
| GET | /email | Yes | Get info of VATUSA email address assigned for user. |
| PUT | /email | Yes | Modify email account. |
| GET | /email/(address) | Yes | Get info of VATUSA email address. |
| GET | /email/hosted | Yes | Get VATUSA hosted email accounts. |
| PUT | /email/hosted/{fac}/{username} | Yes | Modify VATUSA hosted email account. |
| DELETE | /email/hosted/{fac}/{username} | Yes | Delete VATUSA hosted email account. |

# Facility

| Method | Route | Private | Description |
| ------ | ----- | ------- | ----------- |
| GET | /facility |  | Get list of VATUSA facilities. |
| GET | /facility/{id} |  | Get facility information. |
| PUT | /facility/{id} |  | Update facility information. |
| GET | /facility/{id}/email/{templateName} |  | Get facility's email template. |
| POST | /facility/{id}/email/{templateName} |  | Modify facility's email template. |
| POST | /facility/{id}/roster/manageVisitor/{cid} |  | Add member to visiting roster. |
| DELETE | /facility/{id}/roster/manageVisitor/{cid} |  | Delete member from visiting roster. |
| DELETE | /facility/{id}/roster/{cid} |  | Delete member from facility roster. |
| GET | /facility/{id}/roster/{membership} |  | Get facility roster. |
| GET | /facility/{id}/transfers |  | Get pending transfers. |
| PUT | /facility/{id}/transfers/{transferId} |  | Modify transfer request. |

# Infrastructure

| Method | Route | Private | Description |
| ------ | ----- | ------- | ----------- |
| GET | /infrastructure/deploy |  | Deploy Stack. CORS Restricted |
| POST | /infrastructure/deploy |  | Deploy Stack. CORS Restricted |

# Public

| Method | Route | Private | Description |
| ------ | ----- | ------- | ----------- |
| GET | /public/events/(limit) |  | Get events. |
| GET | /public/news/(limits) |  | Get news. |
| GET | /public/planes |  | Get planes for TMU. |

# Solo

| Method | Route | Private | Description |
| ------ | ----- | ------- | ----------- |
| GET | /solo |  | Get list of active solo certifications. |
| POST | /solo |  | Submit new solo certification. |
| DELETE | /solo |  | Delete solo certification. |

# Support

| Method | Route | Private | Description |
| ------ | ----- | ------- | ----------- |
| GET | /support/kb |  | Get knowledgebase list. |
| POST | /support/kb |  | Create knowledgebase category. |
| POST | /support/kb/{categoryId} |  | Create knowledgebase question. |
| PUT | /support/kb/{categoryid}/{questionid} |  | Modify knowledgebase question. |
| DELETE | /support/kb/{categoryid}/{questionid} |  | Delete knowledgebase question. |
| PUT | /support/kb/{id} |  | Modify knowledgebase category. |
| DELETE | /support/kb/{id} |  | Delete knowledgebase category. |
| GET | /support/tickets/depts |  | Get list of assignable departments. |
| GET | /support/tickets/depts/{dept}/staff |  | Get list of assignable staff members for department. |

# Survey

| Method | Route | Private | Description |
| ------ | ----- | ------- | ----------- |
| GET | /survey/{id} | Yes | Get survey questions. |
| POST | /survey/{id} | Yes | Submit survey. |
| POST | /survey/{id}/assign/{cid} | Yes | Assign a survey to cid. |

# TMU

| Method | Route | Private | Description |
| ------ | ----- | ------- | ----------- |
| PUT | /tmu/notice/(id) |  | Edit TMU Notice. |
| DELETE | /tmu/notice/(id) |  | Delete TMU Notice. |
| GET | /tmu/notice/{id} |  | Get TMU Notice info. |
| POST | /tmu/notices |  | Add new TMU Notice. |
| GET | /tmu/notices/(tmufacid?) |  | Get list of TMU Notices. |

# Training

| Method | Route | Private | Description |
| ------ | ----- | ------- | ----------- |
| GET | /facility/{facility}/training/records |  | Get facility's training records. |
| GET | /training/evals | Yes | Get all OTS evaluations. |
| GET | /training/otsEval/{recordID} | Yes | Get OTS Eval content. |
| GET | /training/record/{recordID} |  | Get training record. |
| DELETE | /training/record/{recordID} |  | Delete training record. |
| GET | /training/record/{recordID}/otsEval | Yes | Get attached OTS eval. |
| PUT | /training/record/{record} |  | Edit training record. |
| GET | /training/records | Yes | Get all training records. |
| POST | /user/{cid}/training/otsEval | Yes | Post new OTS Eval for a user. |
| GET | /user/{cid}/training/otsEvals | Yes | Get user's OTS evaluations. |
| POST | /user/{cid}/training/record |  | Submit new training record. |
| GET | /user/{cid}/training/records |  | Get user's training records. |

# User

| Method | Route | Private | Description |
| ------ | ----- | ------- | ----------- |
| GET | /user/(cid) |  | Get user's information. |
| GET | /user/(cid)/log | Yes | Get controller's action log. |
| POST | /user/(cid)/log | Yes | Submit entry to controller's action log. |
| POST | /user/(cid)/rating |  | Submit rating change. |
| GET | /user/(cid)/rating/history |  | Get user's rating history. |
| POST | /user/(cid)/roles/(facility)/(role) |  | Assign new role. |
| DELETE | /user/(cid)/roles/(facility)/(role) |  | Delete role. |
| POST | /user/(cid)/transfer | Yes | Submit transfer request. |
| GET | /user/(cid)/transfer/checklist |  | Get user's transfer checklist. |
| GET | /user/(cid)/transfer/history |  | Get user's transfer history. |
| GET | /user/filtercid/(partialCid) |  | Filter users by partial CID. |
| GET | /user/filterlname/(partialLName) |  | Filter users by partial last name. |
| GET | /user/roles/(facility)/(role) |  | Get users assigned to specific staff role. |
