<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET GetTicketHistory (1)</name>
   <tag></tag>
   <elementGuidId>2c7dfba6-d647-4dd3-a5ef-acc0a4b48ebe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${TIPWebAPI_Bearer}</value>
      <webElementGuid>34637fdd-00b6-4a8f-977e-29294cfd7feb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>20e3ed5d-d466-4e52-b970-b908dbdc543d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${TIPWebAPI_Url}/api/v2/HelpDesk/GetTicketHistory/${Tag}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TIPWebAPI_Url</defaultValue>
      <description></description>
      <id>0e005c7a-f350-431d-bb4a-0c44c5461587</id>
      <masked>false</masked>
      <name>TIPWebAPI_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Tag</defaultValue>
      <description></description>
      <id>628024b0-7d4b-4b02-a57f-4842b8deee15</id>
      <masked>false</masked>
      <name>Tag</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TIPWebAPI_Bearer</defaultValue>
      <description></description>
      <id>191c0dff-6dee-4a5b-98e1-061adefbec2c</id>
      <masked>false</masked>
      <name>TIPWebAPI_Bearer</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
