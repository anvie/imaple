import ssl
import os

from imapclient import IMAPClient

HOST = "localhost"
USERNAME = "someuser"
PASSWORD = "secret"

ssl_context = ssl.create_default_context(cafile=os.environ['CAFILE'])
# don't check if certificate hostname doesn't match target hostname
ssl_context.check_hostname = False

# don't check if the certificate is trusted by a certificate authority
ssl_context.verify_mode = ssl.CERT_NONE

with IMAPClient(HOST, port=9933, ssl_context=ssl_context) as server:
    server.login(USERNAME, PASSWORD)
    select_info = server.select_folder('INBOX')
    print('%d messages in INBOX' % select_info[b'EXISTS'])
    messages = server.search(['FROM', 'best-friend@domain.com'])
    print("%d messages from our best friend" % len(messages))

    messages.pop()

    for msgid, data in server.fetch(messages, ['ENVELOPE']).items():
        envelope = data[b'ENVELOPE']
        print('ID #%d: "%s" received %s' % (msgid, envelope.subject.decode(), envelope.date))

    server.logout()

