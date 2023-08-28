#[doc = "Register `PTPTSCTRL` reader"]
pub type R = crate::R<PTPTSCTRL_SPEC>;
#[doc = "Register `PTPTSCTRL` writer"]
pub type W = crate::W<PTPTSCTRL_SPEC>;
#[doc = "Field `TE` reader - Timestamp enable"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - Timestamp enable"]
pub type TE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFCU` reader - Timestamp fine or coarse update"]
pub type TFCU_R = crate::BitReader;
#[doc = "Field `TFCU` writer - Timestamp fine or coarse update"]
pub type TFCU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TI` reader - Timestamp initialize"]
pub type TI_R = crate::BitReader;
#[doc = "Field `TI` writer - Timestamp initialize"]
pub type TI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TU` reader - Timestamp update"]
pub type TU_R = crate::BitReader;
#[doc = "Field `TU` writer - Timestamp update"]
pub type TU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TITE` reader - Timestamp interrupt trigger enable"]
pub type TITE_R = crate::BitReader;
#[doc = "Field `TITE` writer - Timestamp interrupt trigger enable"]
pub type TITE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARU` reader - Addend register update"]
pub type ARU_R = crate::BitReader;
#[doc = "Field `ARU` writer - Addend register update"]
pub type ARU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETAF` reader - Enable timestamp for all frames"]
pub type ETAF_R = crate::BitReader;
#[doc = "Field `ETAF` writer - Enable timestamp for all frames"]
pub type ETAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TDBRC` reader - Timestamp digital or binary rollover control"]
pub type TDBRC_R = crate::BitReader;
#[doc = "Field `TDBRC` writer - Timestamp digital or binary rollover control"]
pub type TDBRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPPV2F` reader - Enable PTP packet processing for version2 format"]
pub type EPPV2F_R = crate::BitReader;
#[doc = "Field `EPPV2F` writer - Enable PTP packet processing for version2 format"]
pub type EPPV2F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPPEF` reader - Enable processing of PTP over EMAC frames"]
pub type EPPEF_R = crate::BitReader;
#[doc = "Field `EPPEF` writer - Enable processing of PTP over EMAC frames"]
pub type EPPEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPPFSIP6U` reader - Enable processing of PTP frames sent over IPv6-UDP"]
pub type EPPFSIP6U_R = crate::BitReader;
#[doc = "Field `EPPFSIP6U` writer - Enable processing of PTP frames sent over IPv6-UDP"]
pub type EPPFSIP6U_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPPFSIP4U` reader - Enable processing of PTP frames sent over IPv4-UDP"]
pub type EPPFSIP4U_R = crate::BitReader;
#[doc = "Field `EPPFSIP4U` writer - Enable processing of PTP frames sent over IPv4-UDP"]
pub type EPPFSIP4U_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETSFEM` reader - Enable timestamp snapshot for event message"]
pub type ETSFEM_R = crate::BitReader;
#[doc = "Field `ETSFEM` writer - Enable timestamp snapshot for event message"]
pub type ETSFEM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESFMRTM` reader - Enable snapshot for message relevant to master"]
pub type ESFMRTM_R = crate::BitReader;
#[doc = "Field `ESFMRTM` writer - Enable snapshot for message relevant to master"]
pub type ESFMRTM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPPFTS` reader - Select PTP packet for taking snapshot"]
pub type SPPFTS_R = crate::FieldReader;
#[doc = "Field `SPPFTS` writer - Select PTP packet for taking snapshot"]
pub type SPPFTS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EMAFPFF` reader - Enable MAC address for PTP frame filtering"]
pub type EMAFPFF_R = crate::BitReader;
#[doc = "Field `EMAFPFF` writer - Enable MAC address for PTP frame filtering"]
pub type EMAFPFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timestamp enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp fine or coarse update"]
    #[inline(always)]
    pub fn tfcu(&self) -> TFCU_R {
        TFCU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timestamp initialize"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp update"]
    #[inline(always)]
    pub fn tu(&self) -> TU_R {
        TU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp interrupt trigger enable"]
    #[inline(always)]
    pub fn tite(&self) -> TITE_R {
        TITE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Addend register update"]
    #[inline(always)]
    pub fn aru(&self) -> ARU_R {
        ARU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable timestamp for all frames"]
    #[inline(always)]
    pub fn etaf(&self) -> ETAF_R {
        ETAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp digital or binary rollover control"]
    #[inline(always)]
    pub fn tdbrc(&self) -> TDBRC_R {
        TDBRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable PTP packet processing for version2 format"]
    #[inline(always)]
    pub fn eppv2f(&self) -> EPPV2F_R {
        EPPV2F_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable processing of PTP over EMAC frames"]
    #[inline(always)]
    pub fn eppef(&self) -> EPPEF_R {
        EPPEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable processing of PTP frames sent over IPv6-UDP"]
    #[inline(always)]
    pub fn eppfsip6u(&self) -> EPPFSIP6U_R {
        EPPFSIP6U_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable processing of PTP frames sent over IPv4-UDP"]
    #[inline(always)]
    pub fn eppfsip4u(&self) -> EPPFSIP4U_R {
        EPPFSIP4U_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable timestamp snapshot for event message"]
    #[inline(always)]
    pub fn etsfem(&self) -> ETSFEM_R {
        ETSFEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable snapshot for message relevant to master"]
    #[inline(always)]
    pub fn esfmrtm(&self) -> ESFMRTM_R {
        ESFMRTM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packet for taking snapshot"]
    #[inline(always)]
    pub fn sppfts(&self) -> SPPFTS_R {
        SPPFTS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP frame filtering"]
    #[inline(always)]
    pub fn emafpff(&self) -> EMAFPFF_R {
        EMAFPFF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<PTPTSCTRL_SPEC, 0> {
        TE_W::new(self)
    }
    #[doc = "Bit 1 - Timestamp fine or coarse update"]
    #[inline(always)]
    #[must_use]
    pub fn tfcu(&mut self) -> TFCU_W<PTPTSCTRL_SPEC, 1> {
        TFCU_W::new(self)
    }
    #[doc = "Bit 2 - Timestamp initialize"]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TI_W<PTPTSCTRL_SPEC, 2> {
        TI_W::new(self)
    }
    #[doc = "Bit 3 - Timestamp update"]
    #[inline(always)]
    #[must_use]
    pub fn tu(&mut self) -> TU_W<PTPTSCTRL_SPEC, 3> {
        TU_W::new(self)
    }
    #[doc = "Bit 4 - Timestamp interrupt trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn tite(&mut self) -> TITE_W<PTPTSCTRL_SPEC, 4> {
        TITE_W::new(self)
    }
    #[doc = "Bit 5 - Addend register update"]
    #[inline(always)]
    #[must_use]
    pub fn aru(&mut self) -> ARU_W<PTPTSCTRL_SPEC, 5> {
        ARU_W::new(self)
    }
    #[doc = "Bit 8 - Enable timestamp for all frames"]
    #[inline(always)]
    #[must_use]
    pub fn etaf(&mut self) -> ETAF_W<PTPTSCTRL_SPEC, 8> {
        ETAF_W::new(self)
    }
    #[doc = "Bit 9 - Timestamp digital or binary rollover control"]
    #[inline(always)]
    #[must_use]
    pub fn tdbrc(&mut self) -> TDBRC_W<PTPTSCTRL_SPEC, 9> {
        TDBRC_W::new(self)
    }
    #[doc = "Bit 10 - Enable PTP packet processing for version2 format"]
    #[inline(always)]
    #[must_use]
    pub fn eppv2f(&mut self) -> EPPV2F_W<PTPTSCTRL_SPEC, 10> {
        EPPV2F_W::new(self)
    }
    #[doc = "Bit 11 - Enable processing of PTP over EMAC frames"]
    #[inline(always)]
    #[must_use]
    pub fn eppef(&mut self) -> EPPEF_W<PTPTSCTRL_SPEC, 11> {
        EPPEF_W::new(self)
    }
    #[doc = "Bit 12 - Enable processing of PTP frames sent over IPv6-UDP"]
    #[inline(always)]
    #[must_use]
    pub fn eppfsip6u(&mut self) -> EPPFSIP6U_W<PTPTSCTRL_SPEC, 12> {
        EPPFSIP6U_W::new(self)
    }
    #[doc = "Bit 13 - Enable processing of PTP frames sent over IPv4-UDP"]
    #[inline(always)]
    #[must_use]
    pub fn eppfsip4u(&mut self) -> EPPFSIP4U_W<PTPTSCTRL_SPEC, 13> {
        EPPFSIP4U_W::new(self)
    }
    #[doc = "Bit 14 - Enable timestamp snapshot for event message"]
    #[inline(always)]
    #[must_use]
    pub fn etsfem(&mut self) -> ETSFEM_W<PTPTSCTRL_SPEC, 14> {
        ETSFEM_W::new(self)
    }
    #[doc = "Bit 15 - Enable snapshot for message relevant to master"]
    #[inline(always)]
    #[must_use]
    pub fn esfmrtm(&mut self) -> ESFMRTM_W<PTPTSCTRL_SPEC, 15> {
        ESFMRTM_W::new(self)
    }
    #[doc = "Bits 16:17 - Select PTP packet for taking snapshot"]
    #[inline(always)]
    #[must_use]
    pub fn sppfts(&mut self) -> SPPFTS_W<PTPTSCTRL_SPEC, 16> {
        SPPFTS_W::new(self)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP frame filtering"]
    #[inline(always)]
    #[must_use]
    pub fn emafpff(&mut self) -> EMAFPFF_W<PTPTSCTRL_SPEC, 18> {
        EMAFPFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet PTP time stamp control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptsctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSCTRL_SPEC;
impl crate::RegisterSpec for PTPTSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptsctrl::R`](R) reader structure"]
impl crate::Readable for PTPTSCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptptsctrl::W`](W) writer structure"]
impl crate::Writable for PTPTSCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTPTSCTRL to value 0x2000"]
impl crate::Resettable for PTPTSCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
