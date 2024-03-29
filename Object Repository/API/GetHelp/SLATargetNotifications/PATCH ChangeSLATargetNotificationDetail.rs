<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PATCH ChangeSLATargetNotificationDetail</name>
   <tag></tag>
   <elementGuidId>3aeb7f14-9b15-4f67-8b6c-34ba5408a1db</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n  {\n  \t\&quot;op\&quot;: \&quot;replace\&quot;,\n  \t\&quot;path\&quot;: \&quot;/targetNotificationValue\&quot;,\n  \t\&quot;value\&quot;: 1\n  },\n  {\n  \t\&quot;op\&quot;: \&quot;replace\&quot;,\n  \t\&quot;path\&quot;: \&quot;/targetNotificationComparisonValue\&quot;,\n  \t\&quot;value\&quot;: \&quot;\u003c\&quot;\n  },\n  {\n  \t\&quot;op\&quot;: \&quot;replace\&quot;,\n  \t\&quot;path\&quot;: \&quot;/targetNotificationComparisonTypeID\&quot;,\n  \t\&quot;value\&quot;: 2\n  },\n  {\n  \t\&quot;op\&quot;: \&quot;replace\&quot;,\n  \t\&quot;path\&quot;: \&quot;/lastModifiedByUserAccountID\&quot;,\n  \t\&quot;value\&quot;: 1\n  }\n]\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${HHD_API_Bearer}</value>
      <webElementGuid>e9b7e228-de60-4d03-834e-fff9cf5af5a5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ee7a97f1-047d-4dca-90dc-d971445f3ab2</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${HHD_API_Url}/api/SLATargetNotification/ChangeSLATargetNotificationDetail/14</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Url</defaultValue>
      <description></description>
      <id>237f2eb2-9793-4932-b219-94453bc60a2a</id>
      <masked>false</masked>
      <name>HHD_API_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Bearer</defaultValue>
      <description></description>
      <id>1e01f53e-38a9-45de-a338-35b3279a64b8</id>
      <masked>false</masked>
      <name>HHD_API_Bearer</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
