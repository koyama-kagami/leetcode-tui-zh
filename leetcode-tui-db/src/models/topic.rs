use crate::{errors::DBResult, get_db_client};

use super::{question::DbQuestion, *};

#[native_model(id = 2, version = 1)]
#[native_db]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct DbTopic {
    #[primary_key]
    pub slug: String,
    pub name: String,
}

impl Hash for DbTopic {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.slug.hash(state);
    }
}

impl DbTopic {
    pub fn new(slug: &str) -> Self {
        Self {
            slug: slug.into(),
            name: match slug {
                "hot-100" => "Hot 100",
                "interview-150" => "面试经典 150",
                "all" => "全部题目",
                "unknown" => "未分类",
                _ => slug,
            }
            .into(),
        }
    }

    pub fn fetch_all<'a>() -> DBResult<Vec<DbTopic>> {
        let r = get_db_client().r_transaction()?;
        let x = r.scan().primary::<Self>()?.all().into_iter().collect();
        Ok(x)
    }

    pub fn fetch_questions<'a>(&self) -> DBResult<Vec<DbQuestion>> {
        if let Some(slugs) = crate::study_plans::question_slugs(&self.slug) {
            let all = Self::new("all").fetch_questions()?;
            let mut by_slug: std::collections::HashMap<_, _> = all
                .into_iter()
                .map(|question| (question.title_slug.clone(), question))
                .collect();
            return Ok(slugs
                .iter()
                .filter_map(|slug| by_slug.remove(*slug))
                .collect());
        }
        let q_ids = if self.slug.eq("all") {
            let r = get_db_client().r_transaction()?;
            let mut questions: Vec<DbQuestion> = r.scan().primary::<DbQuestion>()?.all().collect();
            questions.sort();
            return Ok(questions);
        } else {
            TopicQuestionMap::get_all_question_by_topic(self)?
        };
        let mut v = vec![];
        for q_id in q_ids {
            if let Some(available_ques) = DbQuestion::get_question_by_id(q_id)? {
                v.push(available_ques);
            };
        }
        v.sort();
        Ok(v)
    }

    pub fn get_topic_by_slug<'a>(slug: &str) -> DBResult<Self> {
        let r = get_db_client().r_transaction()?;

        Ok(r.get()
            .primary(slug.to_string())?
            .ok_or(crate::errors::DbErr::TopicsNotFoundInDb(slug.to_string()))?)
    }
}
