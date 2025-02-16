---
date: 2024-08-09T22:51:00Z
title: "Ressource"
description: |
    Ressource.
spec:
  blog: false
  project: false
  doc: true
tags:
 - project
 - Kube
 - continuous-update
 - GitOps
techno:
  - Kubernetes
  - ArgoCD
  - Helm
  - Terraform
  - Gitea
  - Tekton
  - Kustomize
---

- [CrowdSec](https://crowdsec.net/)
  - <https://www.crowdsec.net/blog/kubernetes-crowdsec-integration>
  - <https://docs.crowdsec.net/docs/cscli/>
  - <https://medium.com/@ninapepite/mise-en-place-de-crowdsec-avec-ingress-nginx-chez-scaleway-5b6f6c9c5555>
- [List certificat](https://crt.sh/?q=weebo.Fr)
  - Setups wildcard certificat and wildcard DNS entry for admin apps.
- [Switch to ArgoCD](https://argoproj.github.io/argo-cd/)
  - [ArgoCD](https://argoproj.github.io/argo-cd/)
- [Secure Harbor](https://goharbor.io/)
  - No public access, some public image will be put on a public registry like dockerhub or github.
  - Cache of all the docker images except the one that setup the cluster.
- Secure the access
  - [Wireguard](https://www.wireguard.com/)
    - [AdGuard Home](https://adguard.com/en/adguard-home/overview.html)
    - Setup a one Endpoint VPN that will be used to access the cluster. (TCP LoadBalancer and one dns match one user)
- Mesh
  - [Istio](https://istio.io/)
  - [Cilium](https://cilium.io/)
  - [Calico](https://www.projectcalico.org/)
  - [Traefik](https://traefik.io/)
- [Monitoring](https://prometheus.io/)
  - [Grafana](https://grafana.com/)
  - [Loki](https://grafana.com/oss/loki/)
  - [Promtail](https://grafana.com/oss/promtail/)
  - [Alertmanager](https://prometheus.io/docs/alerting/alertmanager/)
  - [Tempo](https://grafana.com/oss/tempo/)
- Perfect dev env
  - [Eclipse Che](https://www.eclipse.org/che/)
  - [Tekton](https://tekton.dev/)
  - [Gitea](https://gitea.io/)
  - [Backstage](https://backstage.io/)
    - Creer template de projet sur Github/Gitea.
  - [ArgoRollouts](https://argoproj.github.io/argo-rollouts/)
- Setup
  - [Ansible](https://www.ansible.com/)
    - Automatiser tout le setup du cluster
    - Automatiser la mise à jour des composants
    - Automatiser la mise en place d'un nouveau projet
      - Création de l'app dans ArgoCD (App principal) / Gitea (Groupe, Repo GitOps, Repo CI/CD) / Backstage / Tekton (Main CI/CD via GitOps) / Harbor (Project) / Grafana (DashBoard Folder) / Auth (création de role (Admin,Dev,Ops)_{Nom de l'app} donnant acces au différente brique) / Namespace (Prod / PreProd / Dev)
  - [Terraform](https://www.terraform.io/)
- Engine ?
  - [K3s](https://k3s.io/)
  - [Kubeadm](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/)
  - [Kubespray](https://kubespray.io/)
  - [RKE](https://rancher.com/docs/rke/latest/en/)
- Security
  - [Falco](https://falco.org/)
  - [Popeye](https://popeyecli.io/)
  - [Open Policy Agent](https://www.openpolicyagent.org/)
  - [Kyverno](https://kyverno.io/)
  - [Gatekeeper](https://www.openpolicyagent.org/docs/latest/kubernetes-introduction/)
  - Forcer un certain nombre de label/annotations sur les pods / namespace / service / ingress / Deployment / StatefulSet / DaemonSet / Job / CronJob
  - Analyse des logs
  - Analyse des images et remonté des vulnérabilités (toutes les nuits) (Dashboard Grafana)
