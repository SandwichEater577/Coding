#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/socket.h>

#define PORT      9090
#define MAX_CLIENTS 32
#define BUF_SIZE  1024
#define NICK_SIZE   32

typedef struct {
    int  fd;
    char nick[NICK_SIZE];
    int  active;
} Client;

static Client         clients[MAX_CLIENTS];
static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;

void broadcast(const char *msg, int except_fd) {
    pthread_mutex_lock(&lock);
    for (int i = 0; i < MAX_CLIENTS; i++) {
        if (clients[i].active && clients[i].fd != except_fd)
            send(clients[i].fd, msg, strlen(msg), 0);
    }
    pthread_mutex_unlock(&lock);
}

void *handle_client(void *arg) {
    int  fd = *(int *)arg;
    free(arg);

    char buf[BUF_SIZE];
    char nick[NICK_SIZE] = "anonymous";
    char out[BUF_SIZE + NICK_SIZE + 8];

    int n = recv(fd, buf, BUF_SIZE - 1, 0);
    if (n > 0) {
        buf[n] = '\0';
        buf[strcspn(buf, "\r\n")] = '\0';
        if (strncmp(buf, "NICK ", 5) == 0)
            strncpy(nick, buf + 5, NICK_SIZE - 1);
    }

    pthread_mutex_lock(&lock);
    for (int i = 0; i < MAX_CLIENTS; i++) {
        if (!clients[i].active) {
            clients[i].fd     = fd;
            clients[i].active = 1;
            strncpy(clients[i].nick, nick, NICK_SIZE - 1);
            break;
        }
    }
    pthread_mutex_unlock(&lock);

    snprintf(out, sizeof(out), "*** %s joined\n", nick);
    printf("%s", out);
    broadcast(out, fd);

    while ((n = recv(fd, buf, BUF_SIZE - 1, 0)) > 0) {
        buf[n] = '\0';
        buf[strcspn(buf, "\r\n")] = '\0';
        if (strlen(buf) == 0) continue;
        snprintf(out, sizeof(out), "[%s] %s\n", nick, buf);
        printf("%s", out);
        fflush(stdout);
        broadcast(out, fd);
    }

    snprintf(out, sizeof(out), "*** %s left\n", nick);
    printf("%s", out);
    broadcast(out, fd);

    pthread_mutex_lock(&lock);
    for (int i = 0; i < MAX_CLIENTS; i++) {
        if (clients[i].fd == fd) {
            clients[i].active = 0;
            break;
        }
    }
    pthread_mutex_unlock(&lock);

    close(fd);
    return NULL;
}

int main(void) {
    int server_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (server_fd < 0) { perror("socket"); return 1; }

    int opt = 1;
    setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));

    struct sockaddr_in addr = {
        .sin_family      = AF_INET,
        .sin_port        = htons(PORT),
        .sin_addr.s_addr = INADDR_ANY
    };

    if (bind(server_fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        perror("bind"); return 1;
    }
    listen(server_fd, 16);
    printf("[server] sluchamy na porcie %d\n", PORT);

    while (1) {
        struct sockaddr_in client_addr;
        socklen_t len = sizeof(client_addr);
        int *fd = malloc(sizeof(int));
        *fd = accept(server_fd, (struct sockaddr *)&client_addr, &len);
        if (*fd < 0) { free(fd); continue; }

        printf("[server] polaczenie od %s\n", inet_ntoa(client_addr.sin_addr));

        pthread_t t;
        pthread_create(&t, NULL, handle_client, fd);
        pthread_detach(t);
    }

    close(server_fd);
    return 0;
}
