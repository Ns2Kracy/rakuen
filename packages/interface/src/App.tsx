import { ChakraProvider } from "@chakra-ui/react";
import theme from "./theme";
import { AppRouter } from "./AppRouter";
import { Suspense } from "react";
import { BrowserRouter } from "react-router-dom";

export default function RakuenInterface() {
	return (
		<BrowserRouter>
			<ChakraProvider theme={theme}>
				<Suspense>
					<AppRouter />
				</Suspense>
			</ChakraProvider>
		</BrowserRouter>
	);
}
