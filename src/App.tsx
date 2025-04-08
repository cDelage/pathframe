import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import HomePage from "./features/home/Home";
import Layout from "./ui/layout/Layout";
import PageDesignSystem from "./features/design-system/PageDesignSystem";
import { Toaster } from "react-hot-toast";
import ErrorFallback from "./ui/layout/ErrorFallback";
import { Suspense } from "react";
import Loader from "./ui/kit/Loader";

const router = createBrowserRouter([
  {
    element: <Layout />,
    errorElement: <ErrorFallback />,
    children: [
      {
        path: "/",
        element: <Suspense fallback={<Loader/>}><HomePage /></Suspense>,
      },
      {
        path: "/design-system/:designSystemPath",
        element: (
          <Suspense fallback={<Loader />}>
            <PageDesignSystem />
          </Suspense>
        ),
      },
    ],
  },
]);

function App() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        staleTime: 60 * 1000,
      },
    },
  });

  return (
    <QueryClientProvider client={queryClient}>
      <ReactQueryDevtools initialIsOpen={false} />
      <Toaster
        position="bottom-right"
        gutter={12}
        containerStyle={{ margin: "8px" }}
        toastOptions={{
          success: { duration: 3000 },
          error: {
            duration: 5000,
          },
          style: {
            fontSize: "16px",
            maxWidth: "500px",
            padding: "16px 24px",
            backgroundColor: "var(--component-bg)",
            color: "--base-text",
          },
        }}
      />
      <RouterProvider router={router} />
    </QueryClientProvider>
  );
}

export default App;
