<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT ChangeSLATargetNotificationDetail</name>
   <tag></tag>
   <elementGuidId>c5eb15f0-a2e3-4e45-8136-b6a8442c7cb6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;slaTargetNotificationDetailID\&quot;: 1,\n\t\&quot;slaTargetNotificationID\&quot;: 1,\n\t\&quot;slaTargetTypeID\&quot;: 2,\n\t\&quot;targetNotificationValue\&quot;: 10,\n\t\&quot;targetNotificationComparisonValue\&quot;: \&quot;\u003e\&quot;,\n\t\&quot;targetNotificationComparisonTypeID\&quot;: 1,\n\t\&quot;lastModifiedByUserAccountID\&quot;: 6\n}&quot;,
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
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${HHD_API_Url}/api/SLATargetNotification/ChangeSLATargetNotificationDetail</restUrl>
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
      <id>938f245b-8891-48d3-9bfe-e1a06517925d</id>
      <masked>false</masked>
      <name>HHD_API_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Bearer</defaultValue>
      <description></description>
      <id>7ebcf784-ca8e-471d-9d8e-37930ead3e0a</id>
      <masked>false</masked>
      <name>HHD_API_Bearer</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
