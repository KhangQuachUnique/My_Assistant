import { createBrowserRouter, Navigate } from "react-router";
import { HomeRoute } from "../routes/home/HomeRoute";
import { NotFoundRoute } from "../routes/not-found/NotFoundRoute";
import { RootLayout } from "../routes/root/RootLayout";

export const router = createBrowserRouter([
  {
    path: "/",
    Component: RootLayout,
    children: [
      { index: true, Component: HomeRoute },
      { path: "home", element: <Navigate to="/" replace /> },
      { path: "*", Component: NotFoundRoute },
    ],
  },
]);
