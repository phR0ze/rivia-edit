// edit
// * insert
// * replace

// 
//   - {edit: /root/.bashrc, regex: '|^(export PATH.*)|\1:/opt/<%=distro%>/bin|'}
//   - {edit: /etc/skel/.bashrc, regex: '|^(export PATH.*)|\1:/opt/<%=distro%>/bin|'}

// {edit: /etc/sudoers, insert: append,  "builder ALL=(ALL) NOPASSWD: ALL"}
// {edit: /root/.bashrc, regex: '|^(export PATH.*)|\1:/opt/<%=distro%>/bin|'}
// {edit: /etc/skel/.bashrc, regex: '|^(export PATH.*)|\1:/opt/<%=distro%>/bin|'}
// {edit: /etc/hosts, insert: append,  '127.0.0.1 localhost'}
//   - edit: /etc/locale.conf insert: append values:
//       - 'LANG=<%=language%>.<%=character_set%>'
//       - 'LANGUAGE=<%=language%>.<%=character_set%>'
//   - {edit: /etc/locale.gen, regex: '|^#(<%=language%>\..*)|\1|'}
//  - {edit: /etc/profile.d/locale.sh, insert: append,  'export LC_COLLATE=C'}
//   - {edit: /etc/profile.d/locale.sh, insert: append,  'export
//     LC_ALL=<%=language%>.<%=character_set%>'}
//   - edit: /etc/lsb-release insert: append values:
//       - 'LSB_VERSION=1.4'
//       - 'DISTRIB_ID=<%=distro%>'
//       - 'DISTRIB_RELEASE=rolling'
// - 'DISTRIB_DESCRIPTION=<%=distro%>'

//       # Minimal amount of swapping without disabling it entirely
//       - {edit: '/etc/sysctl.d/10-<%=distro%>.conf', insert: append,  "vm.swappiness = 1"}
//       # Enable kernel ipv4 forwarding for containers
//       - {edit: '/etc/sysctl.d/10-<%=distro%>.conf', insert: append,  "net.ipv4.ip_forward = 1"}
//       # Disable ipv6 forwarding
//       - {edit: '/etc/sysctl.d/10-<%=distro%>.conf', insert: append, "net.ipv6.conf.all.forwarding
//         = 0"}
//       # Increase the number of user file watches to max
//       - {edit: '/etc/sysctl.d/10-<%=distro%>.conf', insert: append,  "fs.inotify.max_user_watches
//         = 524288"}
