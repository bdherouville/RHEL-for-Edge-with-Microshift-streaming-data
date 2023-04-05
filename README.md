# RHEL-for-Edge-with-Microshift-streaming-data
A RHEL for Edge deployment with Microshift for streaming data analysis with observability


```bash
sudo cp .pull-secret.txt /etc/crio/openshift-pull-secret

```
This line copies the contents of a file named `.pull-secret.txt` to the file `/etc/crio/openshift-pull-secret` with superuser privileges. The `cp` command stands for "copy".
```css
sudo firewall-cmd --permanent --zone=trusted --add-source=10.42.0.0/16
sudo firewall-cmd --permanent --zone=trusted --add-source=169.254.169.1
sudo firewall-cmd --permanent --zone=public --add-port=30000/tcp
sudo firewall-cmd --permanent --zone=public --add-port=32000/tcp

```
These lines add firewall rules to the system with superuser privileges using the `firewall-cmd` command. The first two lines add IP addresses to the "trusted" zone, while the last two lines open TCP ports 30000 and 32000 on the "public" zone.
```css
sudo firewall-cmd --reload

```
This line reloads the firewall rules with the changes made in the previous lines.
```css
sudo systemctl stop --now nginx.service
sudo systemctl disable --now nginx.service

```
These lines stop and disable a service named `nginx.service` using the `systemctl` command with superuser privileges. `systemctl stop` stops the service and `systemctl disable` ensures that it will not be started again at boot time.
```sql
sudo systemctl enable --now microshift.service
sudo systemctl start --now microshift.service

```
These lines enable and start a service named `microshift.service` using the `systemctl` command with superuser privileges.
```bash
mkdir ~/.kube
sudo cat /var/lib/microshift/resources/kubeadmin/kubeconfig > ~/.kube/config

```
These lines create a new directory named `.kube` in the user's home directory using the `mkdir` command, then copy the contents of a file located at `/var/lib/microshift/resources/kubeadmin/kubeconfig` to a file named `config` in the `~/.kube` directory with superuser privileges using the `cat` command and the `>` operator for redirection. This `kubeconfig` file will be used to interact with the Kubernetes API.
