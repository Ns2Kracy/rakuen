import { ThemeConfig, extendTheme } from "@chakra-ui/react";
import { colors } from "./colors";

const config: ThemeConfig = {
	initialColorMode: "dark",
	useSystemColorMode: false,
};

export const theme = extendTheme({
	colors,
	config,
});
