// @flow

type ResetPassword = {|
  page: "RESET_PASSWORD",
  email: string,
  id: string,
|};

type ForgotPassword = {|
  page: "FORGOT_PASSWORD",
|};

type Login = {|
  page: "LOGIN",
|};

type Library = {|
  page: "LIBRARY",
|};

type Albums = {|
  page: "ALBUMS",
|};

type SignUp = {|
  page: "SIGNUP",
|};

type NotFound = {|
  page: "404",
|};

type Album = {|
  page: "ALBUM",
  albumId: number,
  photoId: string | null,
|};

export type Route =
  | Login
  | Library
  | ResetPassword
  | ForgotPassword
  | SignUp
  | Album
  | Albums
  | NotFound;

export const parseRoute = (path: string): Route => {
  const [root, queryString] = path.split("?");
  const params = new URLSearchParams(queryString);
  if (root === "/login") {
    return {
      page: "LOGIN",
    };
  }
  if (root === "/signup") {
    return {
      page: "SIGNUP",
    };
  }
  if (root === "/forgot_password") {
    return {
      page: "FORGOT_PASSWORD",
    };
  }
  if (root === "/password_reset") {
    return {
      page: "RESET_PASSWORD",
      // $FlowFixMe: Flow can't tell that this is required.
      id: params.get("id"),
      // $FlowFixMe: Flow can't tell that this is required.
      email: params.get("email"),
    };
  }
  if (root === "/404") {
    return {
      page: "404",
    };
  }
  if (root === "/album") {
    return {
      page: "ALBUM",
      // $FlowFixMe: Flow can't tell that this is required.
      albumId: parseInt(params.get("id"), 10),
      photoId: params.get("photoId") || null,
    };
  }
  if (root === "/albums") {
    return {
      page: "ALBUMS",
    };
  }
  return {
    page: "LIBRARY",
  };
};

export const getPath = (route: Route) => {
  if (route.page === "LOGIN") {
    return "/login";
  }
  if (route.page === "FORGOT_PASSWORD") {
    return "/forgot_password";
  }
  if (route.page === "RESET_PASSWORD") {
    return `/password_reset?email=${encodeURIComponent(route.email)}&id=${
      route.id
    }`;
  }
  if (route.page === "SIGNUP") {
    return "/signup";
  }
  if (route.page === "404") {
    return "/404";
  }
  if (route.page === "ALBUM") {
    return `/album?id=${route.albumId}${
      route.photoId ? `&photoId=${route.photoId}` : ""
    }`;
  }
  if (route.page === "ALBUMS") {
    return "/albums";
  }
  return "/";
};
