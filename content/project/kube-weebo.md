---
date: 2024-08-09T22:51:00Z
title: "Kube-Weebo"
description: |
    Kube-Weebo is a personal kubernetes cluster that will be used to host my personal project and my personal services.
spec:
  blog: false
  project: true
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

Kube-Weebo is a personal kubernetes cluster that will be used to host my personal project and my personal services. This is actually the 4.4 version of my personal server.

- The first one was a simple server in a room who's only purpose was to learn how to setup an ISP server.
- The second one was a simple server in a DC that was mostly used to help me during my early studies.
- The third one was still a simple server in a DC but based on Docker and Docker-Compose.
- The last one is a kubernetes cluster that will be used to host my personal project and my personal services.

The 4.4 version is the fourth iteration of this project. Like the two previous iteration, this one will use a GitOps approach to manage the cluster. It will be managed by ArgoCD unlike the previous one who was managed by FluxCD.
