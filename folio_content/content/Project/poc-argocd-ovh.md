---
date: 2024-09-20T14:58:00Z
title: "Poc ArgoCD OVH"
description: |
    Poc ArgoCD OVH is a proof of concept that will be used to test the integration of ArgoCD with OVH Kubernetes Engine.
spec:
  blog: true
  project: true
  doc: true
tags:
 - project
 - Kube
 - GitOps
 - OVH
techno:
  - Kubernetes
  - ArgoCD
  - Helm
  - Haproxy
  - OVH
links:
  - name: "Poc ArgoCD OVH"
    url: "https://github.com/batleforc/poc-argocd-ovh"
  - name: "ArgoCD"
    url: "https://argoproj.github.io/argo-cd/"
  - name: "OVH Kubernetes Engine"
    url: "https://www.ovhcloud.com/fr/public-cloud/kubernetes/"
---

## Poc ArgoCD OVH

This project is a proof of concept that will be used to test at the same time the integration of ArgoCD with OVH Kubernetes Engine and the Public Cloud of OVH.

The goal of this project is to have a basic infrastructure (3 node) with a load balancer (MetalLB X Ovh) and a GitOps system (ArgoCD) that will deploy a simple stack.

The simple stack is composed of:

- An Ingress controller (Haproxy)
- A certificate manager (Cert-manager)
- A simple web application (Guestbook)
- An endpoint to acess either the Web app or the ArgoCD UI

The project is available on [Github](https://github.com/batleforc/poc-argocd-ovh)
