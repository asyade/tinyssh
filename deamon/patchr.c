#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>

int main(int ac, char **av) {
	int		input_fd, output_fd;
	char	*buffer;
	size_t	offset, size;

	if (ac != 3)
		printf("usage: patchr [input] [output]");
	if ((input_fd = open(av[1], O_RDONLY)) == -1) {
		printf("%s: file not found !\n", av[1]);
		return (1);		
	}
	if ((output_fd = open(av[2], O_WRONLY)) == -1) {
		printf("%s: file not found !\n", av[2]);
		return (1);
	}
	size = lseek(input_fd, 0, SEEK_END);
	offset = lseek(output_fd, 0, SEEK_END);
	lseek(input_fd, 0, SEEK_SET);
	if (!(buffer = malloc(size))) {
		printf("Can't allocate memory !\n");
		return (2);
	}
	read(input_fd, buffer, size);
	write(output_fd, buffer, size);
	close(input_fd);
	close(output_fd);
	free(buffer);
	printf("Patching done, offset : %ld\n", offset);
}