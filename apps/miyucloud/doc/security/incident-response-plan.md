# Miyukini Cloud - Incident Response Plan

**Standards**: ISO 27001 A.5.24-28, RGPD Art. 33-34, HDS | **Version**: 1.0 | **Date**: 2026-03-20

## 1. Incident Severity Levels

| Level | Description | Response Time | Examples |
|---|---|---|---|
| Critical | Active data breach, system compromise | < 1 hour | Unauthorized data exfiltration, credential leak |
| High | Potential data exposure, service disruption | < 4 hours | Suspicious admin activity, authentication bypass attempt |
| Medium | Security policy violation, anomaly detected | < 24 hours | Repeated failed logins from unusual IPs, MIME bypass attempt |
| Low | Minor security event, informational | < 72 hours | Rate limit triggered, expired certificate warning |

## 2. RGPD 72-Hour Notification Workflow

Per RGPD Art. 33, data breaches must be reported to the supervisory authority within 72 hours.

### Status Flow:
```
detected -> assessed -> notified_authority -> notified_subjects -> resolved
```

### Timeline:
1. **T+0h**: Incident detected (auto or manual) -> status: `detected`
2. **T+4h**: Impact assessment completed -> status: `assessed`
3. **T+72h MAX**: Authority notification (CNIL) -> status: `notified_authority`
4. **If high risk to individuals**: Subject notification -> status: `notified_subjects`
5. **Remediation complete**: -> status: `resolved`

## 3. Detection Sources

- **Audit log anomalies**: Bulk downloads, failed login patterns, privilege escalation
- **Rate limiting**: Login/register endpoint abuse
- **MIME validation**: Blocked executable upload attempts
- **Manual report**: Admin-submitted incident via `/api/admin/incidents/report`

## 4. Response Actions

1. **Contain**: Revoke affected sessions, disable compromised accounts
2. **Investigate**: Query audit.events for affected user/time range
3. **Remediate**: Rotate keys, patch vulnerability, update controls
4. **Document**: Record all actions in auth.security_incidents
5. **Notify**: Follow 72h RGPD workflow if personal data affected
6. **Review**: Update risk register, adjust security controls

## 5. Communication Channels

- **Internal**: `MIYUCLOUD_INCIDENT_WEBHOOK_URL` (Slack/Teams webhook)
- **Email**: `MIYUCLOUD_INCIDENT_EMAIL` (security team)
- **Authority**: CNIL portal for RGPD breach notifications
