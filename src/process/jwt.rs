use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub fn process_jwt_sign(sub: &str, aud: &str, exp: &str) -> Result<String> {
    let exp_duration = Duration::from_std(parse_duration(exp)?)?;
    let exp_time = Utc::now() + exp_duration;

    let claims = Claims {
        sub: sub.to_owned(),
        aud: aud.to_owned(),
        exp: exp_time.timestamp(),
    };

    let encoding_key = EncodingKey::from_secret("secret-key".as_ref());
    let token = encode(&Header::default(), &claims, &encoding_key)?;

    Ok(token)
}

pub fn process_jwt_verify(token: &str) -> Result<()> {
    let decoding_key = DecodingKey::from_secret("secret-key".as_ref());
    let validation = Validation::default();

    decode::<Claims>(token, &decoding_key, &validation)?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    exp: i64,
}

fn parse_duration(duration: &str) -> Result<std::time::Duration> {
    // duration likes: 14d, 23m, 12h, 1s
    let duration = duration.trim();
    if duration.is_empty() {
        return Err(anyhow::anyhow!("Duration is empty"));
    }
    let duration = duration.chars().collect::<Vec<_>>();
    let num = duration[..duration.len() - 1]
        .iter()
        .collect::<String>()
        .parse::<u64>()?;
    let duration = match duration[duration.len() - 1] {
        'd' => std::time::Duration::from_secs(num * 24 * 60 * 60),
        'h' => std::time::Duration::from_secs(num * 60 * 60),
        'm' => std::time::Duration::from_secs(num * 60),
        's' => std::time::Duration::from_secs(num),
        _ => return Err(anyhow::anyhow!("Invalid duration unit")),
    };
    Ok(duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("14d").unwrap().as_secs(), 14 * 24 * 60 * 60);
        assert_eq!(parse_duration("23m").unwrap().as_secs(), 23 * 60);
        assert_eq!(parse_duration("12h").unwrap().as_secs(), 12 * 60 * 60);
        assert_eq!(parse_duration("1s").unwrap().as_secs(), 1);
    }

    #[test]
    fn test_process_jwt_sign() {
        let token = process_jwt_sign("user", "audience", "1d").unwrap();
        process_jwt_verify(&token).unwrap();
    }
}
