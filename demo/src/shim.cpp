#include "shim.h"
#include <antlr4-common.h>
#include <antlr4-runtime.h>
#include <JSONLexer.h>
#include <JSONParser.h>
#include <JSONBaseListener.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <string>
#include <memory>

using namespace antlr4;
using namespace antlr4::tree;

extern "C" {
	bool is_valid(const char *input) {
		try {
			ANTLRInputStream stream(input);
			JSONLexer lexer(&stream);
			CommonTokenStream tokens(&lexer);

			JSONParser parser(&tokens);
			parser.setErrorHandler(std::make_shared<BailErrorStrategy>());
			
			ParseTree *tree = parser.json();
			ParseTreeWalker walker;
			JSONBaseListener listener;
			walker.walk(&listener, tree);
		} catch (std::exception &e) {
			return false;
		}
		return true;
	}
}
