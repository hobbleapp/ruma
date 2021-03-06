//! Endpoints for profile.

use bodyparser;
use iron::status::Status;
use iron::{Chain, Handler, IronError, IronResult, Plugin, Request, Response};

use crate::config::Config;
use crate::db::DB;
use crate::error::ApiError;
use crate::middleware::{AccessTokenAuth, JsonRequest, MiddlewareChain, UserIdParam};
use crate::models::profile::Profile as DataProfile;
use crate::models::user::User;
use crate::modifier::{EmptyResponse, SerializableResponse};

/// The `/profile/:user_id` endpoint.
#[derive(Clone, Copy, Debug)]
pub struct Profile;

/// The body of the response for this API.
#[derive(Clone, Debug, Serialize)]
struct ProfileResponse {
    /// The user's avatar URL if they have set one, otherwise not present.
    avatar_url: Option<String>,
    /// The user's display name if they have set one, otherwise not present.
    displayname: Option<String>,
}

middleware_chain!(Profile, [UserIdParam, AccessTokenAuth]);

impl Handler for Profile {
    fn handle(&self, request: &mut Request<'_, '_>) -> IronResult<Response> {
        request
            .extensions
            .get::<User>()
            .expect("AccessTokenAuth should ensure a user");

        let user_id = request
            .extensions
            .get::<UserIdParam>()
            .expect("UserIdParam should ensure a UserId")
            .clone();

        let connection = DB::from_request(request)?;

        let profile = DataProfile::find_by_uid(&connection, &user_id)?;

        let response = match profile {
            Some(profile) => ProfileResponse {
                avatar_url: profile.avatar_url,
                displayname: profile.displayname,
            },
            None => Err(ApiError::not_found(format!(
                "No profile found for {}",
                user_id
            )))?,
        };

        Ok(Response::with((Status::Ok, SerializableResponse(response))))
    }
}

/// The `/profile/:user_id/avatar_url` endpoint.
#[derive(Clone, Copy, Debug)]
pub struct GetAvatarUrl;

/// The body of the response for this API.
#[derive(Clone, Debug, Serialize)]
struct GetAvatarUrlResponse {
    /// The user's avatar URL.
    avatar_url: String,
}

middleware_chain!(GetAvatarUrl, [UserIdParam, AccessTokenAuth]);

impl Handler for GetAvatarUrl {
    fn handle(&self, request: &mut Request<'_, '_>) -> IronResult<Response> {
        request
            .extensions
            .get::<User>()
            .expect("AccessTokenAuth should ensure a user");

        let user_id = request
            .extensions
            .get::<UserIdParam>()
            .expect("UserIdParam should ensure a UserId")
            .clone();

        let connection = DB::from_request(request)?;

        let profile = DataProfile::find_by_uid(&connection, &user_id)?;

        let response = match profile {
            Some(profile) => match profile.avatar_url {
                Some(avatar_url) => GetAvatarUrlResponse { avatar_url },
                None => Err(ApiError::not_found(format!(
                    "No avatar_url found for {}",
                    user_id
                )))?,
            },
            None => Err(ApiError::not_found(format!(
                "No profile found for {}",
                user_id
            )))?,
        };

        Ok(Response::with((Status::Ok, SerializableResponse(response))))
    }
}

/// The `/profile/:user_id/avatar_url` endpoint.
#[derive(Clone, Copy, Debug)]
pub struct PutAvatarUrl;

/// The body of the request for this API.
#[derive(Clone, Debug, Deserialize)]
struct PutAvatarUrlRequest {
    /// The new avatar URL for this user.
    avatar_url: Option<String>,
}

middleware_chain!(PutAvatarUrl, [JsonRequest, UserIdParam, AccessTokenAuth]);

impl Handler for PutAvatarUrl {
    fn handle(&self, request: &mut Request<'_, '_>) -> IronResult<Response> {
        let avatar_url_request = match request.get::<bodyparser::Struct<PutAvatarUrlRequest>>() {
            Ok(Some(avatar_url_request)) => avatar_url_request,
            Ok(None) | Err(_) => Err(ApiError::bad_json(None))?,
        };

        let connection = DB::from_request(request)?;
        let config = Config::from_request(request)?;

        let user = request
            .extensions
            .get::<User>()
            .expect("AccessTokenAuth should ensure a user")
            .clone();

        let user_id = request
            .extensions
            .get::<UserIdParam>()
            .expect("UserIdParam should ensure a UserId")
            .clone();

        if user_id != user.id {
            let error = ApiError::unauthorized(
                "The given user_id does not correspond to the authenticated user".to_string(),
            );

            return Err(IronError::from(error));
        }

        DataProfile::update_avatar_url(
            &connection,
            &config.domain,
            user_id.clone(),
            avatar_url_request.avatar_url,
        )?;

        DataProfile::update_memberships(&connection, &config.domain, user_id)?;

        Ok(Response::with(EmptyResponse(Status::Ok)))
    }
}

/// The `/profile/:user_id/displayname` endpoint.
#[derive(Clone, Copy, Debug)]
pub struct GetDisplayName;

/// The body of the response for this API.
#[derive(Clone, Debug, Serialize)]
struct GetDisplayNameResponse {
    /// The user's display name.
    displayname: String,
}

middleware_chain!(GetDisplayName, [UserIdParam, AccessTokenAuth]);

impl Handler for GetDisplayName {
    fn handle(&self, request: &mut Request<'_, '_>) -> IronResult<Response> {
        request
            .extensions
            .get::<User>()
            .expect("AccessTokenAuth should ensure a user");

        let user_id = request
            .extensions
            .get::<UserIdParam>()
            .expect("UserIdParam should ensure a UserId")
            .clone();

        let connection = DB::from_request(request)?;

        let profile = DataProfile::find_by_uid(&connection, &user_id)?;

        let response = match profile {
            Some(profile) => match profile.displayname {
                Some(displayname) => GetDisplayNameResponse { displayname },
                None => Err(ApiError::not_found(format!(
                    "No displayname found for {}",
                    user_id
                )))?,
            },
            None => Err(ApiError::not_found(format!(
                "No profile found for {}",
                user_id
            )))?,
        };

        Ok(Response::with((Status::Ok, SerializableResponse(response))))
    }
}

/// The `/profile/:user_id/displayname` endpoint.
#[derive(Clone, Copy, Debug)]
pub struct PutDisplayName;

/// The body of the request for this API.
#[derive(Clone, Debug, Deserialize)]
struct PutDisplayNameRequest {
    /// The new display name for this user.
    displayname: Option<String>,
}

middleware_chain!(PutDisplayName, [JsonRequest, UserIdParam, AccessTokenAuth]);

impl Handler for PutDisplayName {
    fn handle(&self, request: &mut Request<'_, '_>) -> IronResult<Response> {
        let displayname_request = match request.get::<bodyparser::Struct<PutDisplayNameRequest>>() {
            Ok(Some(displayname_request)) => displayname_request,
            Ok(None) | Err(_) => Err(ApiError::bad_json(None))?,
        };

        let connection = DB::from_request(request)?;
        let config = Config::from_request(request)?;

        let user = request
            .extensions
            .get::<User>()
            .expect("AccessTokenAuth should ensure a user")
            .clone();

        let user_id = request
            .extensions
            .get::<UserIdParam>()
            .expect("UserIdParam should ensure a UserId")
            .clone();

        if user_id != user.id {
            let error = ApiError::unauthorized(
                "The given user_id does not correspond to the authenticated user".to_string(),
            );

            return Err(IronError::from(error));
        }

        DataProfile::update_displayname(
            &connection,
            &config.domain,
            user_id.clone(),
            displayname_request.displayname,
        )?;

        DataProfile::update_memberships(&connection, &config.domain, user_id)?;

        Ok(Response::with(EmptyResponse(Status::Ok)))
    }
}

#[cfg(test)]
mod tests {
    use crate::query::SyncOptions;
    use crate::test::Test;
    use iron::status::Status;

    #[test]
    fn get_new_user_profile() {
        let test = Test::new();
        let alice = test.create_user();

        let profile_path = format!(
            "/_matrix/client/r0/profile/{}?access_token={}",
            alice.id, alice.token
        );

        assert_eq!(test.get(&profile_path).status, Status::Ok);
    }

    #[test]
    fn get_displayname_non_existent_user() {
        let test = Test::new();
        let carl = test.create_user();
        let user_id = "@carls:ruma.test";

        let get_displayname_path = format!(
            "/_matrix/client/r0/profile/{}/displayname?access_token={}",
            user_id, carl.token
        );

        let response = test.get(&get_displayname_path);

        assert_eq!(response.status, Status::NotFound);
        assert_eq!(
            response.json().get("error").unwrap().as_str().unwrap(),
            format!("No profile found for {}", user_id)
        );
    }

    #[test]
    fn get_avatar_url_non_existent_user() {
        let test = Test::new();
        let carl = test.create_user();
        let user_id = "@carls:ruma.test";

        let get_avatar_url = format!(
            "/_matrix/client/r0/profile/{}/avatar_url?access_token={}",
            user_id, carl.token
        );

        let response = test.get(&get_avatar_url);

        assert_eq!(response.status, Status::NotFound);
        assert_eq!(
            response.json().get("error").unwrap().as_str().unwrap(),
            format!("No profile found for {}", user_id)
        );
    }

    #[test]
    fn put_avatar_url() {
        let test = Test::new();
        let carl = test.create_user();

        let put_avatar_url_path = format!(
            "/_matrix/client/r0/profile/{}/avatar_url?access_token={}",
            carl.id, carl.token
        );
        let response = test.put(
            &put_avatar_url_path,
            r#"{"avatar_url": "mxc://matrix.org/wefh34uihSDRGhw34"}"#,
        );

        assert_eq!(response.status, Status::Ok);

        let get_avatar_url_path = format!(
            "/_matrix/client/r0/profile/{}/avatar_url?access_token={}",
            carl.id, carl.token,
        );
        let response = test.get(&get_avatar_url_path);
        assert_eq!(response.status, Status::Ok);
        assert_eq!(
            response.json().get("avatar_url").unwrap().as_str().unwrap(),
            r#"mxc://matrix.org/wefh34uihSDRGhw34"#
        );
    }

    #[test]
    fn put_displayname() {
        let test = Test::new();
        let carl = test.create_user();

        let put_displayname_path = format!(
            "/_matrix/client/r0/profile/{}/displayname?access_token={}",
            carl.id, carl.token
        );
        let response = test.put(&put_displayname_path, r#"{"displayname": "Bogus"}"#);

        assert_eq!(response.status, Status::Ok);

        let get_displayname_path = format!(
            "/_matrix/client/r0/profile/{}/displayname?access_token={}",
            carl.id, carl.token,
        );
        let response = test.get(&get_displayname_path);
        assert_eq!(response.status, Status::Ok);
        assert_eq!(
            response
                .json()
                .get("displayname")
                .unwrap()
                .as_str()
                .unwrap(),
            r#"Bogus"#
        );
    }

    #[test]
    fn put_displayname_unauthorized() {
        let test = Test::new();
        let bob = test.create_user();
        let alice = test.create_user();

        let put_displayname = format!(
            "/_matrix/client/r0/profile/{}/displayname?access_token={}",
            alice.id, bob.token,
        );

        let response = test.put(&put_displayname, r#"{"displayname": "Alice"}"#);

        assert_eq!(response.status, Status::Forbidden);
        assert_eq!(
            response.json().get("error").unwrap().as_str().unwrap(),
            "The given user_id does not correspond to the authenticated user"
        );
    }

    #[test]
    fn put_avatar_url_unauthorized() {
        let test = Test::new();
        let bob = test.create_user();
        let alice = test.create_user();

        let put_avatar_url = format!(
            "/_matrix/client/r0/profile/{}/avatar_url?access_token={}",
            alice.id, bob.token,
        );

        let response = test.put(
            &put_avatar_url,
            r#"{"avatar_url": "mxc://matrix.org/wefh34uihSDRGhw34"}"#,
        );

        assert_eq!(response.status, Status::Forbidden);
        assert_eq!(
            response.json().get("error").unwrap().as_str().unwrap(),
            "The given user_id does not correspond to the authenticated user"
        );
    }

    #[test]
    fn get_profile() {
        let test = Test::new();
        let carl = test.create_user();

        let avatar_url_body = r#"{"avatar_url": "mxc://matrix.org/some/url"}"#;
        let avatar_url_path = format!(
            "/_matrix/client/r0/profile/{}/avatar_url?access_token={}",
            carl.id, carl.token
        );

        assert!(test
            .put(&avatar_url_path, avatar_url_body)
            .status
            .is_success());

        let displayname_body = r#"{"displayname": "Carl"}"#;
        let displayname_path = format!(
            "/_matrix/client/r0/profile/{}/displayname?access_token={}",
            carl.id, carl.token
        );

        assert!(test
            .put(&displayname_path, displayname_body)
            .status
            .is_success());

        let profile_path = format!(
            "/_matrix/client/r0/profile/{}?access_token={}",
            carl.id, carl.token
        );

        let response = test.get(&profile_path);

        assert_eq!(response.status, Status::Ok);
        assert_eq!(
            response.json().get("avatar_url").unwrap().as_str().unwrap(),
            "mxc://matrix.org/some/url"
        );
        assert_eq!(
            response
                .json()
                .get("displayname")
                .unwrap()
                .as_str()
                .unwrap(),
            "Carl"
        );
    }

    #[test]
    fn get_profile_non_existent_user() {
        let test = Test::new();
        let carl = test.create_user();
        let user_id = "@carls:ruma.test";

        let get_profile = format!(
            "/_matrix/client/r0/profile/{}?access_token={}",
            user_id, carl.token,
        );

        let response = test.get(&get_profile);

        assert_eq!(response.status, Status::NotFound);
        assert_eq!(
            response.json().get("error").unwrap().as_str().unwrap(),
            format!("No profile found for {}", user_id)
        );
    }

    #[test]
    fn update_presence_after_changed_avatar_url() {
        let test = Test::new();
        let carl = test.create_user();

        let presence_list_path = format!(
            "/_matrix/client/r0/presence/list/{}?access_token={}",
            carl.id, carl.token
        );
        let response = test.post(
            &presence_list_path,
            &format!(r#"{{"invite":["{}"], "drop": []}}"#, carl.id),
        );
        assert_eq!(response.status, Status::Ok);

        let avatar_url_body = r#"{"avatar_url": "mxc://matrix.org/some/url"}"#;
        let avatar_url_path = format!(
            "/_matrix/client/r0/profile/{}/avatar_url?access_token={}",
            carl.id, carl.token
        );
        assert!(test
            .put(&avatar_url_path, avatar_url_body)
            .status
            .is_success());

        test.update_presence(&carl.token, &carl.id, r#"{"presence":"online"}"#);

        let options = SyncOptions {
            filter: None,
            since: None,
            full_state: false,
            set_presence: None,
            timeout: 0,
        };
        let response = test.sync(&carl.token, options);
        let array = response
            .json()
            .get("presence")
            .unwrap()
            .get("events")
            .unwrap()
            .as_array()
            .unwrap();
        let mut events = array.iter();
        assert_eq!(events.len(), 1);

        let event = events.next().unwrap();
        assert_eq!(event.get("sender").unwrap().as_str().unwrap(), carl.id);

        let content = event.get("content").unwrap();

        assert_eq!(
            content.get("avatar_url").unwrap().as_str().unwrap(),
            "mxc://matrix.org/some/url"
        );

        let next_batch = Test::get_next_batch(&response);

        let avatar_url_body = r#"{"avatar_url": "mxc://matrix.org/some/new"}"#;
        let avatar_url_path = format!(
            "/_matrix/client/r0/profile/{}/avatar_url?access_token={}",
            carl.id, carl.token
        );
        assert!(test
            .put(&avatar_url_path, avatar_url_body)
            .status
            .is_success());

        let options = SyncOptions {
            filter: None,
            since: Some(next_batch),
            full_state: false,
            set_presence: None,
            timeout: 0,
        };

        let response = test.sync(&carl.token, options);
        let array = response
            .json()
            .get("presence")
            .unwrap()
            .get("events")
            .unwrap()
            .as_array()
            .unwrap();
        let mut events = array.iter();
        assert_eq!(events.len(), 1);

        let event = events.next().unwrap();
        assert_eq!(event.get("sender").unwrap().as_str().unwrap(), carl.id);

        let content = event.get("content").unwrap();

        assert_eq!(
            content.get("avatar_url").unwrap().as_str().unwrap(),
            "mxc://matrix.org/some/new"
        );
    }

    #[test]
    fn update_presence_after_changed_displayname() {
        let test = Test::new();
        let carl = test.create_user();

        let presence_list_path = format!(
            "/_matrix/client/r0/presence/list/{}?access_token={}",
            carl.id, carl.token
        );
        let response = test.post(
            &presence_list_path,
            &format!(r#"{{"invite":["{}"], "drop": []}}"#, carl.id),
        );
        assert_eq!(response.status, Status::Ok);

        let put_displayname_path = format!(
            "/_matrix/client/r0/profile/{}/displayname?access_token={}",
            carl.id, carl.token
        );
        assert!(test
            .put(&put_displayname_path, r#"{"displayname": "Alice"}"#)
            .status
            .is_success());

        test.update_presence(&carl.token, &carl.id, r#"{"presence":"online"}"#);

        let options = SyncOptions {
            filter: None,
            since: None,
            full_state: false,
            set_presence: None,
            timeout: 0,
        };

        let response = test.sync(&carl.token, options);
        let array = response
            .json()
            .get("presence")
            .unwrap()
            .get("events")
            .unwrap()
            .as_array()
            .unwrap();
        let mut events = array.iter();
        assert_eq!(events.len(), 1);

        let event = events.next().unwrap();
        assert_eq!(event.get("sender").unwrap().as_str().unwrap(), carl.id);

        let content = event.get("content").unwrap();

        assert_eq!(
            content.get("displayname").unwrap().as_str().unwrap(),
            "Alice"
        );

        let next_batch = Test::get_next_batch(&response);

        let put_displayname_path = format!(
            "/_matrix/client/r0/profile/{}/displayname?access_token={}",
            carl.id, carl.token
        );
        assert!(test
            .put(&put_displayname_path, r#"{"displayname": "Bogus"}"#)
            .status
            .is_success());

        let options = SyncOptions {
            filter: None,
            since: Some(next_batch),
            full_state: false,
            set_presence: None,
            timeout: 0,
        };

        let response = test.sync(&carl.token, options);
        let array = response
            .json()
            .get("presence")
            .unwrap()
            .get("events")
            .unwrap()
            .as_array()
            .unwrap();
        let mut events = array.iter();
        assert_eq!(events.len(), 1);

        let event = events.next().unwrap();
        assert_eq!(event.get("sender").unwrap().as_str().unwrap(), carl.id);

        let content = event.get("content").unwrap();

        assert_eq!(
            content.get("displayname").unwrap().as_str().unwrap(),
            "Bogus"
        );
    }
}
