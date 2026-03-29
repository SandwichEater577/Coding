#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/socket.h>

#define PORT     9090
#define BUF_SIZE 1024

static int sock_fd;

void *recv_thread(void *arg) {
    (void)arg;
    char buf[BUF_SIZE];
    int  n;
    while ((n = recv(sock_fd, buf, BUF_SIZE - 1, 0)) > 0) {
        buf[n] = '\0';
        printf("%s", buf);
        fflush(stdout);
    }
    printf("[rozlaczono z serwerem]\n");
    exit(0);
    return NULL;
}

int main(int argc, char *argv[]) {
    if (argc < 3) {
        fprintf(stderr, "uzycie: %s <ip> <nick>\n", argv[0]);
        return 1;
    }

    const char *ip   = argv[1];
    const char *nick = argv[2];

    sock_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (sock_fd < 0) { perror("socket"); return 1; }

    struct sockaddr_in addr = {
        .sin_family = AF_INET,
        .sin_port   = htons(PORT)
    };
    if (inet_pton(AF_INET, ip, &addr.sin_addr) <= 0) {
        fprintf(stderr, "zly adres IP: %s\n", ip);
        return 1;
    }

    if (connect(sock_fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        perror("connect");
        return 1;
    }

    char nick_msg[BUFSIZ];
    snprintf(nick_msg, sizeof(nick_msg), "NICK %s\n", nick);
    send(sock_fd, nick_msg, strlen(nick_msg), 0);

    printf("[polaczono jako %s z %s:%d]\n", nick, ip, PORT);

    pthread_t t;
    pthread_create(&t, NULL, recv_thread, NULL);
    pthread_detach(t);

    char buf[BUF_SIZE];
    while (fgets(buf, BUF_SIZE, stdin)) {
        buf[strcspn(buf, "\n")] = '\0';
        if (strlen(buf) == 0) continue;
        char out[BUF_SIZE + 2];
        snprintf(out, sizeof(out), "%s\n", buf);
        send(sock_fd, out, strlen(out), 0);
    }

    close(sock_fd);
    return 0;
}
