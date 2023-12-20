import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.waitForElementPresent(findTestObject('Parts/Add Record/button_Add Record'), 10)

WebUI.click(findTestObject('Parts/Update/div_Part Number'))

WebUI.setText(findTestObject('Parts/Update/input_Edit Part Number'), number)

WebUI.click(findTestObject('Parts/Update/div_Part Name'))

WebUI.setText(findTestObject('Parts/Update/input_Edit Part Name'), name)

WebUI.click(findTestObject('Parts/Update/div_Part Type'))

WebUI.click(findTestObject('Object Repository/Parts/Part Types/li_AUDIOVISUAL EQUIPMENT - OTHERS'))

WebUI.click(findTestObject('Parts/Update/div_Part Area'))

WebUI.click(findTestObject('Parts/Part Areas/li_Technology Department'))

WebUI.click(findTestObject('Parts/Update/div_Cost'))

WebUI.sendKeys(findTestObject('Parts/Update/input_Edit Cost'), cost)

WebUI.click(findTestObject('Parts/Update/div_Min Stock'))

WebUI.sendKeys(findTestObject('Parts/Update/input_Edit Min Stock'), stock)

WebUI.delay(3)

WebUI.verifyElementNotPresent(findTestObject('Parts/Error Messages/label_Existing PN'), 1, FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementNotPresent(findTestObject('Parts/Error Messages/label_Existing Name'), 1, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Parts/Add Record/button_Save Changes'))

WebUI.delay(2)

