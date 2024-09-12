import "./App.css";
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";
import Editor from "./editor/Editor";
import Welcome from "./welcome/Welcome";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Welcome />,
  },
  {
    path : "/editor",
    element: <Editor/>
  }
]);


function App() {
  return (
    <>
      <RouterProvider router={router} />
    </>
  );
}

export default App;
