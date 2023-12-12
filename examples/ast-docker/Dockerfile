FROM alpine:latest

# Update package lists and install dependencies
RUN apk add --no-cache \
    asterisk \
    asterisk-sample-config \
    asterisk-sounds-en \
    bash
    

# Copy configuration files
COPY ./ari.conf /etc/asterisk/ari.conf
COPY ./asterisk.conf /etc/asterisk/asterisk.conf
COPY ./extensions.conf /etc/asterisk/extensions.conf
COPY ./http.conf /etc/asterisk/http.conf
COPY ./sip.conf /etc/asterisk/sip.conf
COPY ./pjsip.conf /etc/asterisk/pjsip.conf
COPY ./rtp.conf /etc/asterisk/rtp.conf


# Expose SIP and RTP ports
EXPOSE 5060/udp
EXPOSE 16384-16394/udp