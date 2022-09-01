import { lazy } from "solid-js";
import { Route, Navigate } from "solid-app-router";
import SwaggerUI from "swagger-ui";
import DocsPage from "./DocsPage";

import swagger from "./api/swagger.json";

const Example = lazy(() => import("./Example.mdx"));

const DocsRoutes = (props) => {
  const docsPage = (page) => {
    return <DocsPage page={page} />;
  };

  return (
    <>
      {/* Docs Routes */}
      <Route path="/" element={<Navigate href="/docs/how-to" />} />
      <Route
        path="/how-to"
        element={<Navigate href="/docs/how-to/quick-start" />}
      />
      <Route path="/how-to/quick-start" element={docsPage(true)} />
      <Route
        path="/reference"
        element={<Navigate href="/docs/reference/api" />}
      />
      <Route path="/reference/api" element={docsPage(false)} />
      <Route path="/api" element={<Navigate href="/docs/api/v0" />} />
      <Route
        path="/api/v0"
        element={
          <>
            <div>
              {SwaggerUI({
                dom_id: "#swagger",
                spec: swagger,
                layout: "BaseLayout",
              })}
            </div>
          </>
        }
      />
    </>
  );
};

export default DocsRoutes;
