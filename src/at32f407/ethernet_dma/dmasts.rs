#[doc = "Register `DMASTS` reader"]
pub type R = crate::R<DMASTS_SPEC>;
#[doc = "Register `DMASTS` writer"]
pub type W = crate::W<DMASTS_SPEC>;
#[doc = "Field `TI` reader - Transmit interrupt"]
pub type TI_R = crate::BitReader;
#[doc = "Field `TI` writer - Transmit interrupt"]
pub type TI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPS` reader - Transmit process stopped"]
pub type TPS_R = crate::BitReader;
#[doc = "Field `TPS` writer - Transmit process stopped"]
pub type TPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TBU` reader - Transmit buffer unavailable"]
pub type TBU_R = crate::BitReader;
#[doc = "Field `TBU` writer - Transmit buffer unavailable"]
pub type TBU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TJT` reader - Transmit jabber timeout"]
pub type TJT_R = crate::BitReader;
#[doc = "Field `TJT` writer - Transmit jabber timeout"]
pub type TJT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVF` reader - Receive overflow"]
pub type OVF_R = crate::BitReader;
#[doc = "Field `OVF` writer - Receive overflow"]
pub type OVF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNF` reader - Transmit underflow"]
pub type UNF_R = crate::BitReader;
#[doc = "Field `UNF` writer - Transmit underflow"]
pub type UNF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RI` reader - Receive interrupt"]
pub type RI_R = crate::BitReader;
#[doc = "Field `RI` writer - Receive interrupt"]
pub type RI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBU` reader - Receive buffer unavailable"]
pub type RBU_R = crate::BitReader;
#[doc = "Field `RBU` writer - Receive buffer unavailable"]
pub type RBU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RPS` reader - Receive process stopped"]
pub type RPS_R = crate::BitReader;
#[doc = "Field `RPS` writer - Receive process stopped"]
pub type RPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWT` reader - Receive watchdog timeout"]
pub type RWT_R = crate::BitReader;
#[doc = "Field `RWT` writer - Receive watchdog timeout"]
pub type RWT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETI` reader - Early transmit interrupt"]
pub type ETI_R = crate::BitReader;
#[doc = "Field `ETI` writer - Early transmit interrupt"]
pub type ETI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBEI` reader - Fatal bus error interrupt"]
pub type FBEI_R = crate::BitReader;
#[doc = "Field `FBEI` writer - Fatal bus error interrupt"]
pub type FBEI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERI` reader - Early receive interrupt"]
pub type ERI_R = crate::BitReader;
#[doc = "Field `ERI` writer - Early receive interrupt"]
pub type ERI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AIS` reader - Abnormal interrupt summary"]
pub type AIS_R = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal interrupt summary"]
pub type AIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NIS` reader - Normal interrupt summary"]
pub type NIS_R = crate::BitReader;
#[doc = "Field `NIS` writer - Normal interrupt summary"]
pub type NIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RS` reader - Receive process state"]
pub type RS_R = crate::FieldReader;
#[doc = "Field `TS` reader - Transmit process state"]
pub type TS_R = crate::FieldReader;
#[doc = "Field `EB` reader - Error bits"]
pub type EB_R = crate::FieldReader;
#[doc = "Field `MMI` reader - MAC MMC interrupt"]
pub type MMI_R = crate::BitReader;
#[doc = "Field `MPI` reader - MAC PMT interrupt"]
pub type MPI_R = crate::BitReader;
#[doc = "Field `TTI` reader - Timestamp trigger interrupt"]
pub type TTI_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit underflow"]
    #[inline(always)]
    pub fn unf(&self) -> UNF_R {
        UNF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable"]
    #[inline(always)]
    pub fn rbu(&self) -> RBU_R {
        RBU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive watchdog timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt"]
    #[inline(always)]
    pub fn fbei(&self) -> FBEI_R {
        FBEI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive process state"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit process state"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error bits"]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - MAC MMC interrupt"]
    #[inline(always)]
    pub fn mmi(&self) -> MMI_R {
        MMI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MAC PMT interrupt"]
    #[inline(always)]
    pub fn mpi(&self) -> MPI_R {
        MPI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timestamp trigger interrupt"]
    #[inline(always)]
    pub fn tti(&self) -> TTI_R {
        TTI_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMASTS")
            .field("ti", &format_args!("{}", self.ti().bit()))
            .field("tps", &format_args!("{}", self.tps().bit()))
            .field("tbu", &format_args!("{}", self.tbu().bit()))
            .field("tjt", &format_args!("{}", self.tjt().bit()))
            .field("ovf", &format_args!("{}", self.ovf().bit()))
            .field("unf", &format_args!("{}", self.unf().bit()))
            .field("ri", &format_args!("{}", self.ri().bit()))
            .field("rbu", &format_args!("{}", self.rbu().bit()))
            .field("rps", &format_args!("{}", self.rps().bit()))
            .field("rwt", &format_args!("{}", self.rwt().bit()))
            .field("eti", &format_args!("{}", self.eti().bit()))
            .field("fbei", &format_args!("{}", self.fbei().bit()))
            .field("eri", &format_args!("{}", self.eri().bit()))
            .field("ais", &format_args!("{}", self.ais().bit()))
            .field("nis", &format_args!("{}", self.nis().bit()))
            .field("rs", &format_args!("{}", self.rs().bits()))
            .field("ts", &format_args!("{}", self.ts().bits()))
            .field("eb", &format_args!("{}", self.eb().bits()))
            .field("mmi", &format_args!("{}", self.mmi().bit()))
            .field("mpi", &format_args!("{}", self.mpi().bit()))
            .field("tti", &format_args!("{}", self.tti().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMASTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TI_W<DMASTS_SPEC, 0> {
        TI_W::new(self)
    }
    #[doc = "Bit 1 - Transmit process stopped"]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TPS_W<DMASTS_SPEC, 1> {
        TPS_W::new(self)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TBU_W<DMASTS_SPEC, 2> {
        TBU_W::new(self)
    }
    #[doc = "Bit 3 - Transmit jabber timeout"]
    #[inline(always)]
    #[must_use]
    pub fn tjt(&mut self) -> TJT_W<DMASTS_SPEC, 3> {
        TJT_W::new(self)
    }
    #[doc = "Bit 4 - Receive overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<DMASTS_SPEC, 4> {
        OVF_W::new(self)
    }
    #[doc = "Bit 5 - Transmit underflow"]
    #[inline(always)]
    #[must_use]
    pub fn unf(&mut self) -> UNF_W<DMASTS_SPEC, 5> {
        UNF_W::new(self)
    }
    #[doc = "Bit 6 - Receive interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<DMASTS_SPEC, 6> {
        RI_W::new(self)
    }
    #[doc = "Bit 7 - Receive buffer unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn rbu(&mut self) -> RBU_W<DMASTS_SPEC, 7> {
        RBU_W::new(self)
    }
    #[doc = "Bit 8 - Receive process stopped"]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RPS_W<DMASTS_SPEC, 8> {
        RPS_W::new(self)
    }
    #[doc = "Bit 9 - Receive watchdog timeout"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<DMASTS_SPEC, 9> {
        RWT_W::new(self)
    }
    #[doc = "Bit 10 - Early transmit interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eti(&mut self) -> ETI_W<DMASTS_SPEC, 10> {
        ETI_W::new(self)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fbei(&mut self) -> FBEI_W<DMASTS_SPEC, 13> {
        FBEI_W::new(self)
    }
    #[doc = "Bit 14 - Early receive interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eri(&mut self) -> ERI_W<DMASTS_SPEC, 14> {
        ERI_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<DMASTS_SPEC, 15> {
        AIS_W::new(self)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<DMASTS_SPEC, 16> {
        NIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet DMA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMASTS_SPEC;
impl crate::RegisterSpec for DMASTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasts::R`](R) reader structure"]
impl crate::Readable for DMASTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmasts::W`](W) writer structure"]
impl crate::Writable for DMASTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMASTS to value 0"]
impl crate::Resettable for DMASTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
