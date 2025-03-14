#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

rm -rf $SCRIPT_DIR/kube-prometheus
mkdir -p $SCRIPT_DIR/kube-prometheus
pushd $SCRIPT_DIR/kube-prometheus
echo "*" >.gitignore

curl -L https://github.com/prometheus-operator/kube-prometheus/archive/main.zip -o kube-prometheus-main.zip
tar -xvf kube-prometheus-main.zip
rm kube-prometheus-main.zip

mv kube-prometheus-main/manifests/setup .
mv kube-prometheus-main/manifests .
rm -rf kube-prometheus-main

kubectl create -f setup
while not kubectl get servicemonitors --all-namespaces
    date
    sleep 1
end
kubectl create -f manifests

function daemon
    pushd /tmp
    status job-control full
    set -l name $argv[1]
    if test -f {$name}.pid
        set -l PID (cat {$name}.pid)
        rm -f {$name}.pid {$name}.out
        if test -n "$PID"
            echo "Killing $name with PID $PID"
            kill $PID
        end
    end
    set -l command $argv[2..-1]
    command nohup $command >{$name}.out 2>&1 &
    echo {$name}...
    echo $last_pid >{$name}.pid
    sleep 0.5
    cat {$name}.out
    popd
end

section "forwarding ports"
kubectl \
    --namespace monitoring \
    wait \
    --for condition=ready \
    --timeout 60s \
    pod \
    --selector app.kubernetes.io/part-of=kube-prometheus
daemon prometheus kubectl --namespace monitoring port-forward svc/prometheus-k8s 9090
daemon grafana kubectl --namespace monitoring port-forward svc/grafana 3000
daemon alertmanager kubectl --namespace monitoring port-forward svc/alertmanager-main 9093
