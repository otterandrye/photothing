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

type SignUp = {|
  page: "SIGNUP",
|};

type NotFound = {|
  page: "404",
|};

export type Route =
  | Login
  | Library
  | ResetPassword
  | ForgotPassword
  | SignUp
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
      id: params.get("id"),
      email: params.get("email"),
    };
  }
  if (root === "/404") {
    return {
      page: "404",
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
  return "";
};
