import { createAlova } from "alova";
import GlobalFetch from "alova/GlobalFetch";
import ReactHook from "alova/react";

const alovaInstance = createAlova({
	baseURL: "http://localhost:7590/api", // for development
	statesHook: ReactHook,
	requestAdapter: GlobalFetch(),
	responded: {
		onSuccess: async (response) => {
			const json = await response.json();
			return json.data;
		},
		onError: (error) => {
			console.log("Error", error.message);
			return error.message;
		},
	},
	beforeRequest(method) {
		method.config.headers.Authorization = `Bearer ${localStorage.getItem("token")}`;
	},
	errorLogger: (error) => {
		console.log("Error", error.message);
	},
});

export { alovaInstance as r };
