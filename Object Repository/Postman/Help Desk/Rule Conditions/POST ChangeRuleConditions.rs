<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST ChangeRuleConditions</name>
   <tag></tag>
   <elementGuidId>7c37a38e-81cf-474f-8ce6-20767c41cecf</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n\t{\n\t\t\&quot;ruleConditionID\&quot;: 2018,\n\t\t\&quot;comparisonTypeID\&quot;: 3,\n\t\t\&quot;comparisonValue\&quot;: null,\n\t\t\&quot;fieldID\&quot;: 102,\n\t\t\&quot;fieldOrigin\&quot;: \&quot;Tag\&quot;,\n\t\t\&quot;order\&quot;: 1,\n\t\t\&quot;comparisonValueListSelections\&quot;: [\n\t\t\t\&quot;279\&quot;,\n\t\t\t\&quot;280\&quot;\n\t\t]\n\t},\n\t{\n\t\t\&quot;ruleConditionID\&quot;: 0,\n\t\t\&quot;comparisonTypeID\&quot;: 3,\n\t\t\&quot;comparisonValue\&quot;: null,\n\t\t\&quot;fieldID\&quot;: 5,\n\t\t\&quot;fieldOrigin\&quot;: \&quot;System\&quot;,\n\t\t\&quot;order\&quot;: 2,\n\t\t\&quot;comparisonValueListSelections\&quot;: [\n\t\t\t\&quot;1\&quot;\n\t\t]\n\t},\n\t{\n\t\t\&quot;ruleConditionID\&quot;: 0,\n\t\t\&quot;comparisonTypeID\&quot;: 1,\n\t\t\&quot;comparisonValue\&quot;: \&quot;Laptop\&quot;,\n\t\t\&quot;fieldID\&quot;: 2,\n\t\t\&quot;fieldOrigin\&quot;: \&quot;System\&quot;,\n\t\t\&quot;order\&quot;: 3,\n\t\t\&quot;comparisonValueListSelections\&quot;: null\n\t}\n]&quot;,
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
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${HHD_API_Url}/api/AutomationRules/2007/RuleConditions/ChangeRuleConditions</restUrl>
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
      <id>7a567806-c09f-4dd3-89a5-ae9a47812662</id>
      <masked>false</masked>
      <name>HHD_API_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Bearer</defaultValue>
      <description></description>
      <id>11ed8398-ce50-45fa-aa37-4613c30728bb</id>
      <masked>false</masked>
      <name>HHD_API_Bearer</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
