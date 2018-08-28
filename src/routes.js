// @flow

type ResetPassword = {|
  page: "RESET_PASSWORD",
|};

type Login = {|
  page: "LOGIN",
|};

type Library = {|
  page: "LIBRARY",
|};

type NotFound = {|
  page: "404",
|};

export type Route = Login | Library | NotFound | ResetPassword;

// This needs the query params too
export const parseRoute = (path: string): Route => {
  if (path === "/login") {
    return {
      page: "LOGIN",
    };
  }
  if (path === "/password_reset") {
    return {
      page: "RESET_PASSWORD",
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
  if (route.page === "RESET_PASSWORD") {
    return "/password_reset";
  }
  return "";
};
