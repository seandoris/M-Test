#include <stdio.h>
#include <libvirt/libvirt.h>

char * state_name(int state_num){
  switch(state_num){
    case VIR_DOMAIN_NOSTATE:
      return "No State";
    case VIR_DOMAIN_RUNNING:
      return "Running";
    case VIR_DOMAIN_BLOCKED:
      return "Blocked";
    case VIR_DOMAIN_PAUSED:
      return "Paused";
    case VIR_DOMAIN_SHUTDOWN:
      return "Shutdown";
    case VIR_DOMAIN_SHUTOFF:
      return "Shutoff";
    case VIR_DOMAIN_CRASHED:
      return "Crashed";
    case VIR_DOMAIN_PMSUSPENDED:
      return "PM Suspended";
    default:
      return "Unexpect State";
  }
}

int main()
{
  int ret;
  virConnectPtr conn;
  virDomainPtr *domains;
  virNetwork;
  conn = virConnectOpenReadOnly("qemu:///system");
  ret = virConnectListAllDomains(conn, &domains,0)
  if (ret < 0) {
    return ret;
  }
  int dstates [ret];
  char *dnames [ret];
  int if_err;
  printf(" Domain Name\tState\n________________________\n");
  for (i = 0; i < ret; i++){
    if_err = virDomainGetState(domains[i],dstates[i],NULL,0);
    if (if_err == -1){
      printf("Unable to obtain domain state");
      return -1;
    }
    char * s_name = state_name(dstates[i]);
    dnames[i] = virDomainGetName(domains[i]);
    printf(" %s\t%s\n",dnames[i],s_name);
  } 

  virConnectClose(conn);
  return ret;
}