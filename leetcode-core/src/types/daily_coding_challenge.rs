use super::problemset_question_list::Question;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub date: String,
    #[serde(
        default,
        deserialize_with = "super::problemset_question_list::null_string"
    )]
    pub user_status: String,
    #[serde(default)]
    pub link: String,
    pub question: Question,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDailyCodingChallengeQuestion {
    pub active_daily_coding_challenge_question: Data,
}

#[derive(Debug)]
pub struct IDailyCodingChallenge {
    pub data: ActiveDailyCodingChallengeQuestion,
}

impl<'de> Deserialize<'de> for IDailyCodingChallenge {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let data = &value["data"];
        let question = data
            .get("activeDailyCodingChallengeQuestion")
            .or_else(|| {
                data.get("todayRecord")
                    .and_then(|r| r.as_array())
                    .and_then(|r| r.first())
            })
            .ok_or_else(|| serde::de::Error::custom("每日一题暂不可用"))?;
        let challenge =
            serde_json::from_value(question.clone()).map_err(serde::de::Error::custom)?;
        Ok(Self {
            data: ActiveDailyCodingChallengeQuestion {
                active_daily_coding_challenge_question: challenge,
            },
        })
    }
}
