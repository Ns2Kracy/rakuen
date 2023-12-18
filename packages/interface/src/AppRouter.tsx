import { lazy } from "react";
import { Route, Routes } from "react-router-dom";

const AppLayout = lazy(() => import("./layouts/AppLayout"));

export const AppRouter = () => {
	return (
		<Routes>
			<Route path={"/"} element={<AppLayout />} />
		</Routes>
	);
};
