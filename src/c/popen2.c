#include <stdio.h>
#include <string.h>

char	*popen2(const char *cmd) {
	FILE *file = popen(cmd, "w");
	char buffer[128000];
	fgets(buffer, 127999, file);
	return (strdup(buffer));
}
