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

WebUI.callTestCase(findTestCase('Logging/Sign In'), [('url') : GlobalVariable.url, ('corpus_admin') : GlobalVariable.corpus_admin
        , ('corpus_pass') : GlobalVariable.corpus_pass], FailureHandling.STOP_ON_FAILURE)

f = new File('C://Temp/perf.csv')

WebUI.delay(7)

WebUI.click(findTestObject('Ticket Perf/b_Tickets'))

WebUI.click(findTestObject('Ticket Perf/a_All Tickets'))

WebUI.delay(20, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.selectOptionByValue(findTestObject('Ticket Perf/select_View All Tickets View Active Tickets_0ca392'), '2', true)

WebUI.delay(7)

WebUI.refresh()

for (int i = 0; i < 50; i++) {
    long a1 = System.currentTimeMillis()

    WebUI.waitForElementClickable(findTestObject('Ticket Perf/grid/div_Open'), 60, FailureHandling.STOP_ON_FAILURE)

    long a2 = System.currentTimeMillis()

    (((f << new Date(a1).format('yyyy-MM-dd HH:mm:ss.SSS')) << ',') << ((a2 - a1) / 1000)) << '\r\n'

    WebUI.refresh()
}

WebUI.click(findTestObject('Log Out/svg_Hayes Support_svg-inline--fa fa-user-circle fa-w-16 gh-blue-icon-color'))

WebUI.click(findTestObject('Log Out/a_Log Out'))

WebUI.closeBrowser()

