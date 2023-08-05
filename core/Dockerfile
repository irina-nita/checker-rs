FROM rust:1.67

# Create new user.
RUN groupadd -r restricted && useradd -r -g restricted sandbox

# Restrict access to almost all
RUN chmod 740 /bin /boot /dev /etc /media /mnt /opt /proc /root /run /sbin /srv /tmp /usr /var

RUN mkdir -p /home/sandbox
RUN mkdir -p /home/sandbox/tests
RUN mkdir -p /home/sandbox/tests/in
RUN mkdir -p /home/sandbox/tests/ref
RUN mkdir -p /home/sandbox/tests/out
RUN mkdir -p /home/sandbox/src
RUN touch /home/sandbox/script.sh

RUN echo "echo "Hello"" > /home/sandbox/script.sh

# Chown on /usr/src/core/submission
RUN chown -R root:restricted /home/sandbox

# Make it read-only for restricted
RUN chmod 740 /home/sandbox

CMD ["/home/sandbox/script.sh"]
